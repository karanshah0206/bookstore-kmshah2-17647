// Bookstore Books Service
//! Book-specific Endpoint Handlers.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use axum::{
  Json, Router,
  extract::{Path, State},
  http::StatusCode,
  routing::{get, post, put},
};
use sqlx::{Error, query, query_as};

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
  State(db_connection): State<MySqlConnectionState>,
  Json(payload): Json<Book>,
) -> Result<Json<Book>, StatusCode> {
  let summary = "Windows 10".to_string();

  match query(
    r#"
    INSERT INTO books
    (isbn, title, author, description, genre, price, quantity, summary)
    VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
  )
  .bind(&payload.isbn)
  .bind(&payload.title)
  .bind(&payload.author)
  .bind(&payload.description)
  .bind(&payload.genre)
  .bind(&payload.price)
  .bind(&payload.quantity)
  .bind(&summary)
  .execute(&db_connection.pool)
  .await
  {
    Ok(_) => Ok(Json(payload)),
    Err(Error::Database(db_error)) if db_error.is_unique_violation() => {
      Err(StatusCode::UNPROCESSABLE_ENTITY)
    }
    Err(e) => {
      eprintln!("{e}");
      Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
  }
}

/// Handler to update book record keyed on ISBN in database.
async fn update_book(
  State(db_connection): State<MySqlConnectionState>,
  Path(isbn): Path<String>,
  Json(payload): Json<Book>,
) -> Result<Json<Book>, StatusCode> {
  match query(
    r#"
  UPDATE books SET
  isbn = ?, title = ?, author = ?, description = ?, genre = ?, price = ?, quantity = ?
  WHERE isbn = ?
  "#,
  )
  .bind(&payload.isbn)
  .bind(&payload.title)
  .bind(&payload.author)
  .bind(&payload.description)
  .bind(&payload.genre)
  .bind(&payload.price)
  .bind(&payload.quantity)
  .bind(&isbn)
  .execute(&db_connection.pool)
  .await
  {
    Ok(result) if result.rows_affected() == 0 => Err(StatusCode::NOT_FOUND),
    Ok(_) => Ok(Json(payload)),
    Err(Error::Database(ref db_error)) if db_error.is_unique_violation() => {
      Err(StatusCode::UNPROCESSABLE_ENTITY)
    }
    Err(e) => {
      eprintln!("{e}");
      Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
  }
}

/// Handler to fetch book record keyed on ISBN from database.
async fn fetch_book(
  State(db_connection): State<MySqlConnectionState>,
  Path(isbn): Path<String>,
) -> Result<Json<BookWithSummary>, StatusCode> {
  match query_as::<_, BookWithSummary>(r#"SELECT * FROM books WHERE isbn = ?"#)
    .bind(&isbn)
    .fetch_one(&db_connection.pool)
    .await
  {
    Ok(book) => Ok(Json(book)),
    Err(Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
    Err(e) => {
      eprintln!("{e}");
      Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
  }
}
