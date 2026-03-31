// Bookstore Web App Service
//! Top-level Environment and Service Driver.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

mod dto;
mod endpoint;
mod middleware;
mod state;

use crate::endpoint::{books, customers, status};
use crate::middleware::auth;

/// Initialize the service routes and execute the service.
#[tokio::main]
async fn main() {
  let public_endpoints = status::get_router();
  let protected_endpoints = books::get_router()
    .merge(customers::get_router())
    .route_layer(axum::middleware::from_fn(auth::validate_jwt));
  let endpoints = public_endpoints.merge(protected_endpoints);

  let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();

  axum::serve(listener, endpoints).await.unwrap();
}
