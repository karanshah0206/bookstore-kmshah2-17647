// Bookstore Web App Service
//! Book-specific Endpoint Handlers.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use axum::{
  Json, Router,
  extract::{Path, rejection::JsonRejection},
  http::StatusCode,
  routing::{get, post, put},
};
use validator::Validate;

use crate::dto::{book::*, failure::*};

/// Construct and return a router for all book-specific endpoints.
pub fn get_router() -> Router {
  Router::new()
    .route("/books", post(create_book))
    .route("/books/{isbn}", put(update_book))
    .route("/books/{isbn}", get(fetch_book))
    .route("/books/isbn/{isbn}", get(fetch_book))
}

/// Handler to enter a new book in the registry.
async fn create_book(
  payload: Result<Json<Book>, JsonRejection>,
) -> Result<(StatusCode, Json<Book>), (StatusCode, Json<Failure>)> {
  let payload = match payload {
    Ok(payload) => payload,
    Err(_) => {
      return Err((
        StatusCode::BAD_REQUEST,
        Json(Failure::new("Badly formatted request body.".to_string())),
      ));
    }
  };

  if let Err(e) = payload.validate() {
    return Err((
      StatusCode::BAD_REQUEST,
      Json(Failure {
        message: e.to_string(),
      }),
    ));
  }

  todo!();
}

/// Handler to update book details using an ISBN key.
async fn update_book(
  Path(isbn): Path<String>,
  payload: Result<Json<Book>, JsonRejection>,
) -> Result<(StatusCode, Json<Book>), (StatusCode, Json<Failure>)> {
  if isbn.is_empty() {
    return Err((
      StatusCode::BAD_REQUEST,
      Json(Failure {
        message: "ISBN parameter is required.".to_string(),
      }),
    ));
  }

  let payload = match payload {
    Ok(payload) => payload,
    Err(_) => {
      return Err((
        StatusCode::BAD_REQUEST,
        Json(Failure::new("Badly formatted request body.".to_string())),
      ));
    }
  };

  if let Err(e) = payload.validate() {
    return Err((
      StatusCode::BAD_REQUEST,
      Json(Failure {
        message: e.to_string(),
      }),
    ));
  }

  todo!();
}

/// Handler to fetch book details using an ISBN key.
async fn fetch_book(
  Path(isbn): Path<String>,
) -> Result<(StatusCode, Json<BookWithSummary>), StatusCode> {
  if isbn.is_empty() {
    return Err(StatusCode::BAD_REQUEST);
  }

  todo!();
}
