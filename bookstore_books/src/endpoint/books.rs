// Bookstore Books Service
//! Book-specific Endpoint Handlers.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use axum::{
  Json, Router,
  extract::{Path, State},
  http::StatusCode,
  routing::{get, post, put},
};
use reqwest::Client;
use sqlx::{Error, query, query_as};
use std::time::Duration;
use tokio::time::timeout;

use crate::dto::{book::*, gemini::*};
use crate::state::{circuit_breaker::*, mysql::MySqlConnectionState};

const RECOMMENDATION_TIMEOUT: Duration = Duration::from_secs(3);

/// Construct and return a router for all book-specific endpoints.
pub fn get_router() -> Router<MySqlConnectionState> {
  Router::new()
    .route("/books", post(create_book))
    .route("/books/{isbn}", put(update_book))
    .route("/books/{isbn}", get(fetch_book))
    .route("/books/{isbn}/related-books", get(fetch_related_books))
}

/// Handler to create a new book record in the database.
async fn create_book(
  State(db_connection): State<MySqlConnectionState>,
  Json(payload): Json<Book>,
) -> Result<Json<Book>, StatusCode> {
  let mut summary = gemini_generate_summary(&payload).await.unwrap_or_else(|e| {
    eprintln!("Failed to generate Gemini summary: {e}");
    fallback_summary(&payload)
  });

  while summary.split(' ').count() < 200 {
    let content = summary.clone();
    summary.push_str(&content);
  }

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

/// Handler to fetch book recommendations from external service based on ISBN key.
async fn fetch_related_books(
  Path(isbn): Path<String>,
) -> Result<Json<Vec<ShortBookResponse>>, StatusCode> {
  if isbn.is_empty() {
    return Err(StatusCode::BAD_REQUEST);
  }

  let is_probe_after_open_window = match check_circuit() {
    CircuitDecision::Reject => return Err(StatusCode::SERVICE_UNAVAILABLE),
    CircuitDecision::Allow {
      is_probe_after_open_window,
    } => is_probe_after_open_window,
  };

  let recommendation_endpoint =
    std::env::var("RECOMMENDATION_ENDPOINT").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
  let request_url = format!("{recommendation_endpoint}/recommended-titles/isbn/{isbn}");

  let response = match timeout(
    RECOMMENDATION_TIMEOUT,
    Client::new().get(request_url).send(),
  )
  .await
  {
    Ok(Ok(response)) => response,
    Ok(Err(e)) => {
      eprintln!("Recommendation service request failed: {e}");
      return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Err(_) => {
      open_circuit();
      return Err(if is_probe_after_open_window {
        StatusCode::SERVICE_UNAVAILABLE
      } else {
        StatusCode::GATEWAY_TIMEOUT
      });
    }
  };

  close_circuit();

  let status = response.status();
  if status == StatusCode::NOT_FOUND {
    Err(StatusCode::NO_CONTENT)
  } else if status == StatusCode::NO_CONTENT || !status.is_success() {
    Err(status)
  } else {
    match response.json::<Vec<ShortBookResponse>>().await {
      Ok(books) => Ok(Json(books)),
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
  }
}

/// Attempt generating a book summary using Gemini.
async fn gemini_generate_summary(book: &Book) -> Result<String, String> {
  let api_key = std::env::var("GEMINI_API_KEY")
    .map_err(|_| "GEMINI_API_KEY environment variable is not set".to_string())?;

  let prompt = format!(
    "You are a book-summary API. The summary MUST be at least 200 words. Never ask for more details. Never mention missing information. If fields look generic, still produce a plausible concise summary using available genre/description context.\n\nTitle: {}\nAuthor: {}\nGenre: {}\nDescription: {}",
    book.title, book.author, book.genre, book.description
  );

  let request_body = GeminiRequest {
    contents: vec![GeminiContent {
      parts: vec![GeminiPart { text: prompt }],
    }],
    generation_config: GeminiGenerationConfig {
      temperature: 0.2,
      max_output_tokens: 120,
    },
  };

  let endpoint = format!(
    "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
    "gemini-2.5-flash-lite", api_key
  );

  let response = Client::new()
    .post(endpoint)
    .json(&request_body)
    .send()
    .await
    .map_err(|e| format!("Gemini request failed: {e}"))?;

  let status = response.status();
  if !status.is_success() {
    let body = response
      .text()
      .await
      .unwrap_or_else(|_| "Unable to read Gemini error response".to_string());
    return Err(format!("Gemini returned status {status}: {body}"));
  }

  let gemini_response: GeminiResponse = response
    .json()
    .await
    .map_err(|e| format!("Failed to parse Gemini response: {e}"))?;

  gemini_response
    .candidates
    .as_ref()
    .and_then(|candidates| candidates.first())
    .and_then(|candidate| candidate.content.as_ref())
    .and_then(|content| content.parts.as_ref())
    .and_then(|parts| parts.first())
    .and_then(|part| part.text.as_deref())
    .map(|text| text.trim().to_string())
    .filter(|text| !text.is_empty())
    .ok_or_else(|| "Gemini returned an empty summary".to_string())
}

/// Use a fixed summary structure as backup if LLM generation fails.
fn fallback_summary(book: &Book) -> String {
  format!(
    "{} by {} is a {} book. {}",
    book.title,
    book.author,
    book.genre,
    book.description.chars().take(180).collect::<String>()
  )
}
