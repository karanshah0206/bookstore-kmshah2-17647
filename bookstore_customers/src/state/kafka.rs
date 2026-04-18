// Bookstore Customers Service
//! Kafka producer state for customer domain events.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use std::env;

use rdkafka::{
  ClientConfig,
  error::KafkaError,
  producer::{FutureProducer, FutureRecord},
  util::Timeout,
};

use crate::dto::customer::CustomerWithId;

const DEFAULT_KAFKA_BOOTSTRAP_SERVERS: &str =
  "98.88.99.206:9092,34.195.107.7:9092,54.221.160.63:9092";
const CUSTOMER_TOPIC: &str = "kmshah2.customer.evt";

/// Kafka producer wrapper for customer domain events.
#[derive(Clone)]
pub struct KafkaProducerState {
  pub producer: FutureProducer,
}

impl KafkaProducerState {
  /// Create a Kafka producer with the configured bootstrap servers.
  pub fn new() -> Self {
    let bootstrap_servers = env::var("KAFKA_BOOTSTRAP_SERVERS")
      .unwrap_or_else(|_| DEFAULT_KAFKA_BOOTSTRAP_SERVERS.to_string());

    KafkaProducerState {
      producer: ClientConfig::new()
        .set("bootstrap.servers", &bootstrap_servers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Failed to create Kafka producer."),
    }
  }

  /// Publish the customer registered domain event.
  pub async fn publish_customer_registered_event(
    &self,
    customer: &CustomerWithId,
  ) -> Result<(), KafkaError> {
    let payload = serde_json::to_string(customer).expect("Failed to serialize customer event.");

    self
      .producer
      .send(
        FutureRecord::to(CUSTOMER_TOPIC)
          .key(&customer.user_id)
          .payload(&payload),
        Timeout::Never,
      )
      .await
      .map_err(|(error, _)| error)?;

    Ok(())
  }
}
