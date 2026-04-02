// Bookstore Web BFF Service
//! Data Transfer Objects for Book Entities.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Number;
use validator::Validate;

/// Schema for book entity requests/responses.
#[derive(Deserialize, Serialize, Validate)]
pub struct Book {
  #[serde(rename = "ISBN")]
  #[validate(length(min = 1))]
  pub isbn: String,
  #[validate(length(min = 1))]
  pub title: String,
  #[serde(rename = "Author")]
  #[validate(length(min = 1))]
  pub author: String,
  #[validate(length(min = 1))]
  pub description: String,
  #[validate(length(min = 1))]
  pub genre: String,
  #[serde(deserialize_with = "deserialize_price")]
  #[validate(range(min = 0.0))]
  pub price: f64,
  pub quantity: u64,
}

/// Schema for book entity requests/responses with a summary.
#[derive(Deserialize, Serialize)]
pub struct BookWithSummary {
  #[serde(rename = "ISBN")]
  pub isbn: String,
  pub title: String,
  #[serde(rename = "Author")]
  pub author: String,
  pub description: String,
  pub genre: String,
  #[serde(deserialize_with = "deserialize_price")]
  pub price: f64,
  pub quantity: u64,
  pub summary: String,
}

/// Custom deserializer for the price attribute to validate 0-2 decimal places.
fn deserialize_price<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
  D: Deserializer<'de>,
{
  let num: Number = Deserialize::deserialize(deserializer)?;
  let price_str = num.to_string();

  if let Some(dot_index) = price_str.find('.') {
    let decimal_count = price_str.len() - dot_index - 1;

    if decimal_count > 2 {
      return Err(serde::de::Error::custom(format!(
        "Price {price_str} has {decimal_count} decimal places; must be at most 2."
      )));
    }
  }

  num
    .as_f64()
    .ok_or_else(|| serde::de::Error::custom("Badly formatted price attribute."))
}
