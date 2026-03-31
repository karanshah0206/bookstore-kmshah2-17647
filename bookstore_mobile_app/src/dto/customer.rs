// Bookstore Mobile App Service
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

/// Schema for customer entity with ID requests/responses.
#[derive(Deserialize, Serialize)]
pub struct CustomerWithId {
  pub id: u64,
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
