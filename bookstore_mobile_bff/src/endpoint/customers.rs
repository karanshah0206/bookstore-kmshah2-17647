// Bookstore Mobile BFF Service
//! Customer-specific Endpoint Handlers.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use axum::{
  Json, Router,
  extract::{Path, Query, State, rejection::JsonRejection},
  http::StatusCode,
  routing::{get, post},
};
use validator::Validate;

use crate::dto::{customer::*, failure::*};
use crate::state::tcp::HttpConnectionState;

/// Construct and return a router for all customer-specific endpoints.
pub fn get_router() -> Router<HttpConnectionState> {
  Router::new()
    .route("/customers", post(create_customer))
    .route("/customers", get(fetch_customer_by_user_id))
    .route("/customers/{id}", get(fetch_customer_by_id))
}

/// Handler to enter a new customer in the registry.
async fn create_customer(
  State(alb_conn_state): State<HttpConnectionState>,
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

  match alb_conn_state
    .http_client
    .post(alb_conn_state.endpoint_url + "/customers")
    .json(&payload.0)
    .send()
    .await
  {
    Ok(response) => {
      if response.status().is_success() {
        match response.json::<CustomerWithId>().await {
          Ok(customer) => Ok((StatusCode::OK, Json(customer))),
          _ => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Failure {
              message: "Invalid customer JSON received from internal service.".to_string(),
            }),
          )),
        }
      } else {
        Err((
          response.status(),
          Json(Failure {
            message: match response.status() {
              StatusCode::UNPROCESSABLE_ENTITY => "This user ID already exists in the system.",
              _ => "Error in creating a customer entry.",
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

/// Handler to fetch customer details using an ID key.
async fn fetch_customer_by_id(
  State(alb_conn_state): State<HttpConnectionState>,
  Path(id): Path<u64>,
) -> Result<(StatusCode, Json<CustomerNoAddress>), StatusCode> {
  match alb_conn_state
    .http_client
    .get(alb_conn_state.endpoint_url + "/customers/id/" + &id.to_string())
    .send()
    .await
  {
    Ok(response) => {
      let status = response.status();
      if status.is_success() {
        match response.json::<CustomerWithId>().await {
          Ok(customer) => Ok((
            StatusCode::OK,
            Json(CustomerNoAddress::from_cust_with_id(customer)),
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

/// Handler to fetch customer details using a user ID key.
async fn fetch_customer_by_user_id(
  State(alb_conn_state): State<HttpConnectionState>,
  Query(params): Query<UserIdQuery>,
) -> Result<(StatusCode, Json<CustomerNoAddress>), StatusCode> {
  if params.validate().is_err() {
    return Err(StatusCode::BAD_REQUEST);
  }

  match alb_conn_state
    .http_client
    .get(alb_conn_state.endpoint_url + "/customers/userId/" + &params.user_id.to_string())
    .send()
    .await
  {
    Ok(response) => {
      let status = response.status();
      if status.is_success() {
        match response.json::<CustomerWithId>().await {
          Ok(customer) => Ok((
            StatusCode::OK,
            Json(CustomerNoAddress::from_cust_with_id(customer)),
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
