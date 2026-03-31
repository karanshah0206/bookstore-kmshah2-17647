// Bookstore Mobile BFF Service
//! Data Transfer Objects for Failures.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use serde::Serialize;

/// Response schema for a failure response.
#[derive(Serialize)]
pub struct Failure {
  pub message: String,
}

impl Failure {
  /// Create a failure response.
  pub fn new(message: String) -> Self {
    Failure { message }
  }
}
