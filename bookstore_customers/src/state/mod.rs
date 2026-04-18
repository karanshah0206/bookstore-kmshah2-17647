// Bookstore Customers Service
//! Application state wrapper for MySQL and Kafka resources.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

pub mod kafka;
pub mod mysql;

use kafka::KafkaProducerState;
use mysql::MySqlConnectionState;

#[derive(Clone)]
pub struct AppState {
  pub mysql: MySqlConnectionState,
  pub kafka: KafkaProducerState,
}

impl AppState {
  pub async fn new() -> Self {
    AppState {
      mysql: MySqlConnectionState::new().await,
      kafka: KafkaProducerState::new(),
    }
  }
}
