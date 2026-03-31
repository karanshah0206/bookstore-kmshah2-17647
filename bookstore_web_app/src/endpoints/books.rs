// Bookstore Web App Service
//! Book-specific endpoint handlers for the Bookstore Web App Service.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use axum::{
  Json,
  extract::{Path, rejection::JsonRejection},
  http::StatusCode,
};
use validator::Validate;

use crate::dto::{book::*, failure::*};

/// Endpoint to enter a new book in the registry.
pub async fn create_book(
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

/// Endpoint to update book details using an ISBN key.
pub async fn update_book(
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

/// Endpoint to fetch book details using an ISBN key.
pub async fn fetch_book(
  Path(isbn): Path<String>,
) -> Result<(StatusCode, Json<BookWithSummary>), StatusCode> {
  if isbn.is_empty() {
    return Err(StatusCode::BAD_REQUEST);
  }

  todo!();
}
