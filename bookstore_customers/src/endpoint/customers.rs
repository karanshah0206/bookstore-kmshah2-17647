// Bookstore Customers Service
//! Customer-specific Endpoint Handlers.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use axum::{
  Json, Router,
  extract::{Path, State},
  http::StatusCode,
  routing::{get, post},
};
use sqlx::{Error, query, query_as};

use crate::{dto::customer::*, state::AppState};

/// Construct and return a router for all customer-specific endpoints.
pub fn get_router() -> Router<AppState> {
  Router::new()
    .route("/customers", post(create_customer))
    .route("/customers/id/{id}", get(fetch_customer_by_id))
    .route(
      "/customers/userId/{user_id}",
      get(fetch_customer_by_user_id),
    )
}

/// Handler to create a new customer record in the database.
async fn create_customer(
  State(app_state): State<AppState>,
  Json(payload): Json<Customer>,
) -> Result<Json<CustomerWithId>, StatusCode> {
  match query(
    r#"
    INSERT INTO customers
    (user_id, name, phone, address_1, address_2, city, state, zipcode)
    VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
  )
  .bind(&payload.user_id)
  .bind(&payload.name)
  .bind(&payload.phone)
  .bind(&payload.address_1)
  .bind(&payload.address_2)
  .bind(&payload.city)
  .bind(&payload.state)
  .bind(&payload.zipcode)
  .execute(&app_state.mysql.pool)
  .await
  {
    Ok(response) => {
      let customer = CustomerWithId::from_customer_add_id(payload, response.last_insert_id());

      if let Err(e) = app_state
        .kafka
        .publish_customer_registered_event(&customer)
        .await
      {
        eprintln!("{e}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
      }

      Ok(Json(customer))
    }
    Err(Error::Database(db_error)) if db_error.is_unique_violation() => {
      Err(StatusCode::UNPROCESSABLE_ENTITY)
    }
    Err(e) => {
      eprintln!("{e}");
      Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
  }
}

/// Handler to fetch customer record keyed on ID from database.
async fn fetch_customer_by_id(
  State(app_state): State<AppState>,
  Path(id): Path<u64>,
) -> Result<Json<CustomerWithId>, StatusCode> {
  match query_as::<_, CustomerWithId>(r#"SELECT * FROM customers WHERE id = ?"#)
    .bind(&id)
    .fetch_one(&app_state.mysql.pool)
    .await
  {
    Ok(customer) => Ok(Json(customer)),
    Err(Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
    Err(e) => {
      eprintln!("{e}");
      Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
  }
}

/// Handler to fetch customer record keyed on user ID from database.
async fn fetch_customer_by_user_id(
  State(app_state): State<AppState>,
  Path(user_id): Path<String>,
) -> Result<Json<CustomerWithId>, StatusCode> {
  match query_as::<_, CustomerWithId>(r#"SELECT * FROM customers WHERE user_id = ?"#)
    .bind(&user_id)
    .fetch_one(&app_state.mysql.pool)
    .await
  {
    Ok(customer) => Ok(Json(customer)),
    Err(Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
    Err(e) => {
      eprintln!("{e}");
      Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
  }
}
