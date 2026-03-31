// Bookstore Books Service
//! Top-level Environment and Service Driver.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

mod dto;
mod endpoint;

use crate::endpoint::{books, status};

#[tokio::main]
async fn main() {
  // Networking constants.
  const BIND_ADDR: &str = "0.0.0.0:3000";

  // Routing service endpoints.
  let endpoints = status::get_router().merge(books::get_router());

  // Binding to target address and port at runtime.
  let listener = tokio::net::TcpListener::bind(BIND_ADDR).await.unwrap();

  // Serve endpoints on bound listener.
  axum::serve(listener, endpoints).await.unwrap();
}
