// Bookstore Books Service
//! Data Transfer Objects for Book Entities.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use serde::{Deserialize, Serialize};

/// Schema for book entity.
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
  pub quantity: u64,
}

/// Schema for book entity with a summary.
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
  pub quantity: u64,
  pub summary: String,
}
