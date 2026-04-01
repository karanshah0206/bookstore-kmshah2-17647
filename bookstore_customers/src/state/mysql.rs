// Bookstore Customers Service
//! State Manager for MySQL Database Connection Pools.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use std::env;

use sqlx::{MySql, Pool, mysql::MySqlPoolOptions};

/// State of a MySQL database connection connection pool.
#[derive(Clone)]
pub struct MySqlConnectionState {
  pub pool: Pool<MySql>,
}

impl MySqlConnectionState {
  /// Establish a new MySQL connection pool with database and return the connection pool state.
  pub async fn new() -> Self {
    // Database constants.
    const MAX_CONNECTIONS_IN_POOL: u32 = 5;
    let database_endpoint =
      env::var("DATABASE_ENDPOINT").expect("DATABASE_ENDPOINT environment variable must be set.");
    let database_username =
      env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME environment variable must be set.");
    let database_password =
      env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD environment variable must be set.");
    let database_name =
      env::var("DATABASE_NAME").expect("DATABASE_NAME environment variable must be set.");

    MySqlConnectionState {
      pool: MySqlPoolOptions::new()
        .max_connections(MAX_CONNECTIONS_IN_POOL)
        .connect(&format!(
          "mysql://{database_username}:{database_password}@{database_endpoint}/{database_name}"
        ))
        .await
        .expect("Failed to establish connection pool with database."),
    }
  }
}
