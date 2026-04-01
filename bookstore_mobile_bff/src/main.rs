// Bookstore Mobile BFF Service
//! Top-level Environment and Service Driver.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

mod dto;
mod endpoint;
mod middleware;
mod state;

use std::env;

use dotenv::dotenv;

use crate::endpoint::{books, customers, status};
use crate::middleware::auth;
use crate::state::tcp::HttpConnectionState;

/// Initialize the service routes, connect to internal load balancer, and execute the service.
#[tokio::main]
async fn main() {
  // Load environment variables.
  dotenv().ok();

  // Set networking constants.
  let bind_address: String =
    env::var("BIND_ADDRESS").expect("BIND_ADDRESS environment variable must be set.");
  let alb_endpoint: String =
    env::var("INTERNAL_ALB_URL").expect("INTERNAL_ALB_URL environment variable must be set.");
  let alb_connection_timeout: u64 = env::var("ALB_CONNECTION_TIMEOUT")
    .expect("ALB_CONNECTION_TIMEOUT environment variable must be set.")
    .parse()
    .expect("ALB_CONNECTION_TIMEOUT must be of type u64.");

  // Establish stateful connection with internal application load balancer.
  let alb_conn_state = HttpConnectionState::new(alb_endpoint, alb_connection_timeout);

  // Routing service endpoints.
  let public_endpoints = status::get_router();
  let protected_endpoints = books::get_router()
    .merge(customers::get_router())
    .with_state(alb_conn_state)
    .route_layer(axum::middleware::from_fn(auth::validate_jwt));
  let endpoints = public_endpoints.merge(protected_endpoints);

  // Binding to target address and port at runtime.
  let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();

  // Serve endpoints on bound listener.
  axum::serve(listener, endpoints).await.unwrap();
}
