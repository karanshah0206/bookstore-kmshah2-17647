// Bookstore Books Service
//! Book-specific Endpoint Handlers.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use axum::{
  Json, Router,
  extract::{Path, State},
  http::StatusCode,
  routing::{get, post, put},
};

use crate::{dto::book::*, state::mysql::MySqlConnectionState};

/// Construct and return a router for all book-specific endpoints.
pub fn get_router() -> Router<MySqlConnectionState> {
  Router::new()
    .route("/books", post(create_book))
    .route("/books/{isbn}", put(update_book))
    .route("/books/{isbn}", get(fetch_book))
}

/// Handler to create a new book record in the database.
async fn create_book(
  State(pool): State<MySqlConnectionState>,
  Json(payload): Json<Book>,
) -> Result<Json<Book>, StatusCode> {
  todo!();
}

/// Handler to update book record keyed on ISBN in database.
async fn update_book(
  State(pool): State<MySqlConnectionState>,
  Path(isbn): Path<String>,
  Json(payload): Json<Book>,
) -> Result<Json<Book>, StatusCode> {
  todo!();
}

/// Handler to fetch book record keyed on ISBN from database.
async fn fetch_book(
  State(pool): State<MySqlConnectionState>,
  Path(isbn): Path<String>,
) -> Result<Json<BookWithSummary>, StatusCode> {
  todo!();
}
