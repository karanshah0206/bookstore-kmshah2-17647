use axum::{Router, routing::get};

/// Run the 
#[tokio::main]
async fn main() {
  let app = Router::new().route("/status", get(status));

  let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();

  axum::serve(listener, app).await.unwrap();
}

/// Indicate that the service is healthy.
async fn status() -> &'static str {
  "Healthy"
}
