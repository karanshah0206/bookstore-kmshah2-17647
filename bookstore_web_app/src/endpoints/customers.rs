// Bookstore Web App Service
//! Customer-specific endpoint handlers for the Bookstore Web App Service.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use axum::{
  Json,
  extract::{Path, Query},
  http::StatusCode,
};

use crate::dto::{customer::*, failure::*};

/// Endpoint to enter a new customer in the registry.
pub async fn create_customer(
  Json(payload): Json<Customer>,
) -> Result<(StatusCode, Json<CustomerWithId>), (StatusCode, Json<Failure>)> {
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
  todo!();
}
