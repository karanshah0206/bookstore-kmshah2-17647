// Customer CRM Service
//! Data Transfer Object for the customer registration event.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use serde::{Deserialize, Serialize};

/// Customer registered domain event payload.
#[derive(Deserialize, Serialize)]
pub struct CustomerRegisteredEvent {
  pub id: u64,
  #[serde(rename = "userId")]
  pub user_id: String,
  pub name: String,
  pub phone: String,
  #[serde(rename = "address")]
  pub address_1: String,
  #[serde(rename = "address2")]
  pub address_2: Option<String>,
  pub city: String,
  pub state: String,
  pub zipcode: String,
}
