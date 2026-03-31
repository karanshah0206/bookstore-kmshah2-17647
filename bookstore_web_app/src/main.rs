// Bookstore Web App Service
//! Service Driver for the Bookstore Web App Service.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

mod dto;
mod endpoints;

use axum::{
  Router,
  routing::{get, post, put},
};

use crate::endpoints::{books::*, customers::*};

/// Initialize the service routes and execute the service.
#[tokio::main]
async fn main() {
  let app = Router::new()
    .route("/status", get(status))
    .route("/books", post(create_book))
    .route("/books/{isbn}", put(update_book))
    .route("/books/{isbn}", get(fetch_book))
    .route("/books/isbn/{isbn}", get(fetch_book))
    .route("/customers", post(create_customer))
    .route("/customers", get(fetch_customer_by_user_id))
    .route("/customers/{id}", get(fetch_customer_by_id));

  let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();

  axum::serve(listener, app).await.unwrap();
}

/// Endpoint to indicate Indicate that the service is healthy.
async fn status() -> &'static str {
  "OK"
}
