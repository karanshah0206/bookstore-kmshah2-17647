// Bookstore Customers Service
//! Data Transfer Objects for Customer Entities.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Schema for customer entity.
#[derive(Deserialize, Serialize, FromRow)]
pub struct Customer {
  #[serde(rename = "userId")]
  pub user_id: String,
  pub name: String,
  pub phone: String,
  #[serde(rename = "address")]
  pub address_1: String,
  #[serde(rename = "address2")]
  pub address_2: String,
  pub city: String,
  pub state: String,
  pub zipcode: String,
}

/// Schema for customer entity with ID.
#[derive(Deserialize, Serialize, FromRow)]
pub struct CustomerWithId {
  pub id: u64,
  #[serde(rename = "userId")]
  pub user_id: String,
  pub name: String,
  pub phone: String,
  #[serde(rename = "address")]
  pub address_1: String,
  #[serde(rename = "address2")]
  pub address_2: String,
  pub city: String,
  pub state: String,
  pub zipcode: String,
}

impl CustomerWithId {
  pub fn from_customer_add_id(customer: Customer, id: u64) -> Self {
    CustomerWithId {
      id,
      user_id: customer.user_id,
      name: customer.name,
      phone: customer.phone,
      address_1: customer.address_1,
      address_2: customer.address_2,
      city: customer.city,
      state: customer.state,
      zipcode: customer.zipcode,
    }
  }
}
