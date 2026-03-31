// Bookstore Web App Service
//! Data Transfer Objects for Book Entities.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use serde::{Deserialize, Serialize};

/// Expected request schema for creating a new book entity.
#[derive(Deserialize)]
pub struct CreateBookRequest {
  pub isbn: String,
  pub title: String,
  pub author: String,
  pub description: String,
  pub genre: String,
  pub price: f64,
  pub quantity: usize,
}

/// Response schema for successfully creating a new book entity.
#[derive(Serialize)]
pub struct CreatedBookResponse {
  pub isbn: String,
  pub title: String,
  pub author: String,
  pub description: String,
  pub genre: String,
  pub price: f64,
  pub quantity: usize,
}

impl CreatedBookResponse {
  /// Create a book creation success response.
  pub fn new(
    isbn: String,
    title: String,
    author: String,
    description: String,
    genre: String,
    price: f64,
    quantity: usize,
  ) -> Self {
    CreatedBookResponse {
      isbn,
      title,
      author,
      description,
      genre,
      price,
      quantity,
    }
  }
}

/// Response schema for failure in creating a new book entity.
#[derive(Serialize)]
pub struct FailedBookCreationResponse {
  pub message: String,
}

impl FailedBookCreationResponse {
  /// Create a book creation failure response.
  pub fn new(message: String) -> Self {
    FailedBookCreationResponse { message }
  }
}
