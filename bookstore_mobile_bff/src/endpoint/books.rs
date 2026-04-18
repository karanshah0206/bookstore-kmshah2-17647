// Bookstore Mobile BFF Service
//! Book-specific Endpoint Handlers.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use axum::{
  Json, Router,
  extract::{Path, State, rejection::JsonRejection},
  http::StatusCode,
  routing::{get, post, put},
};
use validator::Validate;

use crate::dto::{book::*, failure::*};
use crate::state::tcp::HttpConnectionState;

/// Construct and return a router for all book-specific endpoints.
pub fn get_router() -> Router<HttpConnectionState> {
  Router::new()
    .route("/books", post(create_book))
    .route("/books/{isbn}", put(update_book))
    .route("/books/{isbn}", get(fetch_book))
    .route("/books/isbn/{isbn}", get(fetch_book))
    .route("/books/isbn/{isbn}/related-books", get(fetch_related_books))
}

/// Handler to enter a new book in the registry.
async fn create_book(
  State(alb_conn_state): State<HttpConnectionState>,
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

  if let Err(validation_error) = payload.validate() {
    return Err((
      StatusCode::BAD_REQUEST,
      Json(Failure {
        message: validation_error.to_string(),
      }),
    ));
  }

  match alb_conn_state
    .http_client
    .post(alb_conn_state.endpoint_url + "/books")
    .json(&payload.0)
    .send()
    .await
  {
    Ok(response) => {
      if response.status().is_success() {
        match response.json::<Book>().await {
          Ok(book) => Ok((StatusCode::CREATED, Json(book))),
          _ => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Failure {
              message: "Invalid book JSON received from internal service.".to_string(),
            }),
          )),
        }
      } else {
        Err((
          response.status(),
          Json(Failure {
            message: match response.status() {
              StatusCode::UNPROCESSABLE_ENTITY => "This ISBN already exists in the system.",
              _ => "Error in creating a book entry.",
            }
            .to_string(),
          }),
        ))
      }
    }
    Err(server_error) => Err((
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(Failure {
        message: server_error.to_string(),
      }),
    )),
  }
}

/// Handler to update book details using an ISBN key.
async fn update_book(
  State(alb_conn_state): State<HttpConnectionState>,
  Path(isbn): Path<String>,
  payload: Result<Json<Book>, JsonRejection>,
) -> Result<(StatusCode, Json<Book>), StatusCode> {
  if isbn.is_empty() {
    return Err(StatusCode::BAD_REQUEST);
  }

  let payload = match payload {
    Ok(payload) => payload,
    Err(_) => {
      return Err(StatusCode::BAD_REQUEST);
    }
  };

  if payload.validate().is_err() || payload.isbn != isbn {
    return Err(StatusCode::BAD_REQUEST);
  }

  match alb_conn_state
    .http_client
    .put(alb_conn_state.endpoint_url + "/books/" + &isbn)
    .json(&payload.0)
    .send()
    .await
  {
    Ok(response) => {
      let status = response.status();
      if status.is_success() {
        match response.json::<Book>().await {
          Ok(book) => Ok((StatusCode::OK, Json(book))),
          _ => Err(status),
        }
      } else {
        Err(status)
      }
    }
    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
  }
}

/// Handler to fetch book details using an ISBN key.
async fn fetch_book(
  State(alb_conn_state): State<HttpConnectionState>,
  Path(isbn): Path<String>,
) -> Result<(StatusCode, Json<BookResponse>), StatusCode> {
  if isbn.is_empty() {
    return Err(StatusCode::BAD_REQUEST);
  }

  match alb_conn_state
    .http_client
    .get(alb_conn_state.endpoint_url + "/books/" + &isbn)
    .send()
    .await
  {
    Ok(response) => {
      let status = response.status();
      if status.is_success() {
        match response.json::<BookWithSummary>().await {
          Ok(book) => Ok((
            StatusCode::OK,
            Json(if book.genre == "non-fiction" {
              BookResponse::NumericGenre(BookWithSummaryNumericGenre::non_fiction_from_book(book))
            } else {
              BookResponse::StringGenre(book)
            }),
          )),
          _ => Err(status),
        }
      } else {
        Err(status)
      }
    }
    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
  }
}

/// Handler to fetch related book recommendations using an ISBN key.
async fn fetch_related_books(
  State(alb_conn_state): State<HttpConnectionState>,
  Path(isbn): Path<String>,
) -> Result<(StatusCode, Json<Vec<ShortBookResponse>>), StatusCode> {
  if isbn.is_empty() {
    return Err(StatusCode::BAD_REQUEST);
  }

  match alb_conn_state
    .http_client
    .get(alb_conn_state.endpoint_url + "/books/" + &isbn + "/related-books")
    .send()
    .await
  {
    Ok(response) => {
      let status = response.status();
      if status == StatusCode::NO_CONTENT || !status.is_success() {
        Err(status)
      } else {
        match response.json::<Vec<ShortBookResponse>>().await {
          Ok(books) => Ok((StatusCode::OK, Json(books))),
          _ => Err(status),
        }
      }
    }
    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
  }
}
