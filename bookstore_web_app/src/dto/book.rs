// Bookstore Web App Service
//! Data Transfer Objects for Book Entities.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use serde::{Deserialize, Serialize};

/// Schema for book entity requests/responses.
#[derive(Deserialize, Serialize)]
pub struct Book {
  #[serde(rename = "ISBN")]
  pub isbn: String,
  pub title: String,
  #[serde(rename = "Author")]
  pub author: String,
  pub description: String,
  pub genre: String,
  pub price: f64,
  pub quantity: usize,
}

impl Book {
  /// Create a book entity response.
  pub fn new(
    isbn: String,
    title: String,
    author: String,
    description: String,
    genre: String,
    price: f64,
    quantity: usize,
  ) -> Self {
    Book {
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

/// Schema for book entity requests/responses with a summary.
#[derive(Deserialize, Serialize)]
pub struct BookWithSummary {
  #[serde(rename = "ISBN")]
  pub isbn: String,
  pub title: String,
  #[serde(rename = "Author")]
  pub author: String,
  pub description: String,
  pub genre: String,
  pub price: f64,
  pub quantity: usize,
  pub summary: String,
}

impl BookWithSummary {
  /// Create a book entity response with a summary.
  pub fn new(
    isbn: String,
    title: String,
    author: String,
    description: String,
    genre: String,
    price: f64,
    quantity: usize,
    summary: String,
  ) -> Self {
    BookWithSummary {
      isbn,
      title,
      author,
      description,
      genre,
      price,
      quantity,
      summary,
    }
  }
}
