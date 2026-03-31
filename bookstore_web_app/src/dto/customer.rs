// Bookstore Web App Service
//! Data Transfer Objects for Customer Entities.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

/// Schema for customer entity requests/responses.
#[derive(Deserialize, Serialize, Validate)]
pub struct Customer {
  #[serde(rename = "userId")]
  #[validate(email)]
  pub user_id: String,
  #[validate(length(min = 1))]
  pub name: String,
  #[validate(length(min = 1))]
  pub phone: String,
  #[serde(rename = "address")]
  #[validate(length(min = 1))]
  pub address_1: String,
  #[serde(rename = "address2")]
  #[validate(length(min = 1))]
  pub address_2: String,
  #[validate(length(min = 1))]
  pub city: String,
  #[validate(custom(function = "validate_us_state"))]
  pub state: String,
  #[validate(length(min = 1))]
  pub zipcode: String,
}

impl Customer {
  /// Create a new customer response entity.
  pub fn new(
    user_id: String,
    name: String,
    phone: String,
    address_1: String,
    address_2: String,
    city: String,
    state: String,
    zipcode: String,
  ) -> Self {
    Customer {
      user_id,
      name,
      phone,
      address_1,
      address_2,
      city,
      state,
      zipcode,
    }
  }
}

/// Schema for customer entity with ID requests/responses.
#[derive(Deserialize, Serialize)]
pub struct CustomerWithId {
  pub id: usize,
  #[serde(rename = "userId")]
  pub user_id: String,
  pub name: String,
  pub phone: String,
  #[serde(rename = "address")]
  pub address_1: String,
  #[serde(rename = "address1")]
  pub address_2: String,
  pub city: String,
  pub state: String,
  pub zipcode: String,
}

impl CustomerWithId {
  /// Create a new customer with ID response entity.
  pub fn new(
    id: usize,
    user_id: String,
    name: String,
    phone: String,
    address_1: String,
    address_2: String,
    city: String,
    state: String,
    zipcode: String,
  ) -> Self {
    CustomerWithId {
      id,
      user_id,
      name,
      phone,
      address_1,
      address_2,
      city,
      state,
      zipcode,
    }
  }
}

/// Expected query schema for fetch customer by user ID endpoint.
#[derive(Deserialize, Validate)]
pub struct UserIdQuery {
  #[serde(rename = "userId")]
  #[validate(email)]
  pub user_id: String,
}

/// Custom validator function for US states attribute.
fn validate_us_state(state: &str) -> Result<(), ValidationError> {
  const US_STATES: [&str; 50] = [
    "AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "FL", "GA", "HI", "ID", "IL", "IN", "IA", "KS",
    "KY", "LA", "ME", "MD", "MA", "MI", "MN", "MS", "MO", "MT", "NE", "NV", "NH", "NJ", "NM", "NY",
    "NC", "ND", "OH", "OK", "OR", "PA", "RI", "SC", "SD", "TN", "TX", "UT", "VT", "VA", "WA", "WV",
    "WI", "WY",
  ];

  if US_STATES.contains(&state) {
    Ok(())
  } else {
    Err(ValidationError::new("Invalid US state."))
  }
}
