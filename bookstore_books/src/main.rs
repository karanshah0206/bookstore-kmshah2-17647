// Bookstore Books Service
//! Top-level Environment and Service Driver.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

mod dto;
mod endpoint;
mod state;

use std::env;

use dotenv::dotenv;

use crate::endpoint::{books, status};
use crate::state::mysql::MySqlConnectionState;

/// Initialize the service routes, connect to database, and execute the service.
#[tokio::main]
async fn main() {
  // Load environment variables.
  dotenv().ok();

  // Networking constants.
  let bind_address: String =
    env::var("BIND_ADDRESS").expect("BIND_ADDRESS environment variable must be set.");

  // Establish connection pool with database.
  let connection_pool = MySqlConnectionState::new().await;

  // Routing service endpoints.
  let status_endpoint = status::get_router();
  let books_endpoint = books::get_router().with_state(connection_pool);
  let endpoints = status_endpoint.merge(books_endpoint);

  // Binding to target address and port at runtime.
  let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();

  // Serve endpoints on bound listener.
  axum::serve(listener, endpoints).await.unwrap();
}
