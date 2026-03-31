// Bookstore Web App Service
//! Book-specific endpoint handlers for the Bookstore Web App Service.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use axum::{Json, extract::Path, http::StatusCode};

use crate::dto::{book::*, failure::*};

/// Endpoint to enter a new book in the registry.
pub async fn create_book(
  Json(payload): Json<Book>,
) -> Result<(StatusCode, Json<Book>), (StatusCode, Json<Failure>)> {
  todo!();
}

/// Endpoint to update book details using an ISBN key.
pub async fn update_book(
  Path(isbn): Path<String>,
  Json(payload): Json<Book>,
) -> Result<(StatusCode, Json<Book>), (StatusCode, Json<Failure>)> {
  todo!();
}

/// Endpoint to fetch book details using an ISBN key.
pub async fn fetch_book(
  Path(isbn): Path<String>,
) -> Result<(StatusCode, Json<BookWithSummary>), StatusCode> {
  todo!()
}
