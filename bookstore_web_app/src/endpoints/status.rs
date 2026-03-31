// Bookstore Web App Service
//! Health-check (status) endpoint handler for the Bookstore Web App Service.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use axum::{Router, routing::get};

pub fn get_router() -> Router {
  Router::new().route("/status", get(status))
}

/// Endpoint to indicate Indicate that the service is healthy.
async fn status() -> &'static str {
  "OK"
}
