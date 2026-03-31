// Bookstore Web App Service
//! Data Transfer Objects for Book Entities.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use serde::{Deserialize, Serialize};

/// Schema for book entity requests/responses.
#[derive(Deserialize, Serialize)]
pub struct Book {
  pub isbn: String,
  pub title: String,
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

/// Schema for book entity requests/responses.
#[derive(Deserialize, Serialize)]
pub struct SummarizedBook {
  pub isbn: String,
  pub title: String,
  pub author: String,
  pub description: String,
  pub genre: String,
  pub price: f64,
  pub quantity: usize,
  pub summary: String,
}

impl SummarizedBook {
  /// Create a book entity response.
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
    SummarizedBook {
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
