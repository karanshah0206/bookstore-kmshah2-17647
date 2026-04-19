// Bookstore Books Service
//! Data Transfer Objects for Book Entities.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Schema for book entity.
#[derive(Deserialize, Serialize, FromRow)]
pub struct Book {
  #[serde(rename = "ISBN")]
  pub isbn: String,
  pub title: String,
  #[serde(rename = "Author")]
  pub author: String,
  pub description: String,
  pub genre: String,
  pub price: f64,
  pub quantity: u64,
}

/// Schema for book entity with a summary.
#[derive(Deserialize, Serialize, FromRow)]
pub struct BookWithSummary {
  #[serde(rename = "ISBN")]
  pub isbn: String,
  pub title: String,
  #[serde(rename = "Author")]
  pub author: String,
  pub description: String,
  pub genre: String,
  pub price: f64,
  pub quantity: u64,
  pub summary: String,
}

/// Schema for book recommendations response.
#[derive(Deserialize, Serialize)]
pub struct ShortBookResponseResponse {
  #[serde(rename = "ISBN")]
  pub isbn: String,
  pub title: String,
  #[serde(rename = "Author")]
  pub author: String,
}

/// Schema for book recommendations request from external service.
#[derive(Deserialize, Serialize)]
pub struct ShortBookResponseRequest {
  pub isbn: String,
  pub title: String,
  #[serde(rename = "authors")]
  pub author: String,
  pub publisher: String,
}

impl ShortBookResponseRequest {
  /// Transform book request schema to book response schema.
  pub fn to_response(&self) -> ShortBookResponseResponse {
    ShortBookResponseResponse {
      isbn: self.isbn.clone(),
      title: self.title.clone(),
      author: self.author.clone(),
    }
  }
}

/// Transform the recommendations request vector into response vector.
pub fn recommendations_transformer(
  requests: Vec<ShortBookResponseRequest>,
) -> Vec<ShortBookResponseResponse> {
  requests
    .iter()
    .map(|request| request.to_response())
    .collect()
}
