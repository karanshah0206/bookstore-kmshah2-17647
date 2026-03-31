// Bookstore Web App Service
//! Customer-specific endpoint handlers for the Bookstore Web App Service.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use axum::{
  Json,
  extract::{Path, Query, rejection::JsonRejection},
  http::StatusCode,
};
use validator::Validate;

use crate::dto::{customer::*, failure::*};

/// Endpoint to enter a new customer in the registry.
pub async fn create_customer(
  payload: Result<Json<Customer>, JsonRejection>,
) -> Result<(StatusCode, Json<CustomerWithId>), (StatusCode, Json<Failure>)> {
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

/// Endpoint to fetch customer details using an ID key.
pub async fn fetch_customer_by_id(
  Path(id): Path<usize>,
) -> Result<(StatusCode, Json<CustomerWithId>), StatusCode> {
  todo!();
}

/// Endpoint to fetch customer details using a user ID key.
pub async fn fetch_customer_by_user_id(
  Query(params): Query<UserIdQuery>,
) -> Result<(StatusCode, Json<CustomerWithId>), StatusCode> {
  if params.validate().is_err() {
    return Err(StatusCode::BAD_REQUEST);
  }

  todo!();
}
