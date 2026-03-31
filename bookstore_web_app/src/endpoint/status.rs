// Bookstore Web App Service
//! Health-check (Status) Endpoint Handler.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use axum::{Router, routing::get};

/// Construct and return a router for the health-check endpoint.
pub fn get_router() -> Router {
  Router::new().route("/status", get(status))
}

/// Endpoint to indicate Indicate that the service is healthy.
async fn status() -> &'static str {
  "OK"
}
