// Bookstore Web App Service
//! Data Transfer Objects for Customer Entities.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use serde::{Deserialize, Serialize};

/// Schema for customer entity requests/responses.
#[derive(Deserialize, Serialize)]
pub struct Customer {
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
