// Bookstore Web BFF Service
//! State Manager for Network Connections.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use std::time::Duration;

use reqwest::Client;

/// State of an HTTP connection with a client.
#[derive(Clone)]
pub struct HttpConnectionState {
  pub http_client: Client,
  pub endpoint_url: String,
}

impl HttpConnectionState {
  /// Establish a new HTTP connection with a client and return the connection state.
  pub fn new(endpoint_url: String, timeout: u64) -> Self {
    HttpConnectionState {
      http_client: Client::builder()
        .timeout(Duration::from_secs(timeout))
        .build()
        .unwrap(),
      endpoint_url: endpoint_url.to_string(),
    }
  }
}
