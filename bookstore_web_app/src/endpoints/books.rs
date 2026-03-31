use axum::{Json, extract::Path, http::StatusCode};

use crate::dto::{
  book::{Book, SummarizedBook},
  failure::Failure,
};

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
) -> Result<(StatusCode, Json<SummarizedBook>), StatusCode> {
  todo!()
}
