// Bookstore Books Service
//! Top-level Environment and Service Driver.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

mod dto;
mod endpoint;

use std::env;

use dotenv::dotenv;

use crate::endpoint::{books, status};

/// Initialize the service routes, connect to database, and execute the service.
#[tokio::main]
async fn main() {
  // Load environment variables.
  dotenv().ok();

  // Networking constants.
  let bind_address: String =
    env::var("BIND_ADDRESS").expect("BIND_ADDRESS environment variable must be set.");

  // Database constants.
  let database_endpoint =
    env::var("DATABASE_ENDPOINT").expect("DATABASE_ENDPOINT environment variable must be set.");
  let database_username =
    env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME environment variable must be set.");
  let database_password =
    env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD environment variable must be set.");
  let database_name =
    env::var("DATABASE_NAME").expect("DATABASE_NAME environment variable must be set.");
  let table_name = env::var("TABLE_NAME").expect("TABLE_NAME environment variable must be set.");

  let database_connection_url =
    format!("mysql://{database_username}:{database_password}@{database_endpoint}/{database_name}");

  println!("{database_connection_url}");

  // Routing service endpoints.
  let endpoints = status::get_router().merge(books::get_router());

  // Binding to target address and port at runtime.
  let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();

  // Serve endpoints on bound listener.
  axum::serve(listener, endpoints).await.unwrap();
}
