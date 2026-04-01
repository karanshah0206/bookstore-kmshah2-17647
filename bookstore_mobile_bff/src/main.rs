// Bookstore Mobile BFF Service
//! Top-level Environment and Service Driver.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

mod dto;
mod endpoint;
mod middleware;
mod state;

use crate::endpoint::{books, customers, status};
use crate::middleware::auth;
use crate::state::tcp::HttpConnectionState;

/// Initialize the service routes and execute the service.
#[tokio::main]
async fn main() {
  // Networking constants.
  const BIND_ADDR: &str = "0.0.0.0:80";
  const ALB_ENDPOINT: &str = "http://localhost:3000";
  const ALB_CONN_TIMEOUT: u64 = 10; // seconds

  // Establish stateful connection with internal application load balancer.
  let alb_conn_state = HttpConnectionState::new(ALB_ENDPOINT, ALB_CONN_TIMEOUT);

  // Routing service endpoints.
  let public_endpoints = status::get_router();
  let protected_endpoints = books::get_router()
    .merge(customers::get_router())
    .with_state(alb_conn_state)
    .route_layer(axum::middleware::from_fn(auth::validate_jwt));
  let endpoints = public_endpoints.merge(protected_endpoints);

  // Binding to target address and port at runtime.
  let listener = tokio::net::TcpListener::bind(BIND_ADDR).await.unwrap();

  // Serve endpoints on bound listener.
  axum::serve(listener, endpoints).await.unwrap();
}
