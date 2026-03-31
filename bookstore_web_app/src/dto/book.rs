// Bookstore Web App Service
//! Data Transfer Objects for Book Entities
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use serde::{Deserialize, Serialize};

/// Expected request schema for creating a new book entity.
#[derive(Deserialize)]
pub struct CreateBookRequest {
  isbn: String,
  title: String,
  author: String,
  description: String,
  genre: String,
  price: f64,
  quantity: usize,
}

/// Response schema for successfully creating a new book entity.
#[derive(Serialize)]
pub struct CreatedBookResponse {
  isbn: String,
  title: String,
  author: String,
  description: String,
  genre: String,
  price: f64,
  quantity: usize,
}

/// Response schema for failure in creating a new book entity.
#[derive(Serialize)]
pub struct FailedBookCreationResponse {
  message: String,
}
