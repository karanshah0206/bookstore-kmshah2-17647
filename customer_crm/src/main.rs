// Customer CRM Service
//! Top-level asynchronous consumer and email dispatcher.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

mod dto;
mod mailer;

use std::env;

use anyhow::{Context, Result};
use dotenv::dotenv;
use rdkafka::{
  consumer::{Consumer, StreamConsumer},
  message::Message,
  ClientConfig,
};

use crate::{dto::customer::CustomerRegisteredEvent, mailer::EmailSender};

const DEFAULT_KAFKA_BOOTSTRAP_SERVERS: &str =
  "98.88.99.206:9092,34.195.107.7:9092,54.221.160.63:9092";
const DEFAULT_KAFKA_GROUP_ID: &str = "customer-crm";
const DEFAULT_KAFKA_TOPIC: &str = "kmshah2.customer.evt";

/// Asynchronous CRM service that consumes customer registration events.
pub struct CustomerCrmService {
  consumer: StreamConsumer,
  mailer: EmailSender,
  topic: String,
}

impl CustomerCrmService {
  /// Create the Kafka consumer and email sender.
  pub fn new() -> Result<Self> {
    let bootstrap_servers = env::var("KAFKA_BOOTSTRAP_SERVERS")
      .unwrap_or_else(|_| DEFAULT_KAFKA_BOOTSTRAP_SERVERS.to_string());
    let group_id =
      env::var("KAFKA_GROUP_ID").unwrap_or_else(|_| DEFAULT_KAFKA_GROUP_ID.to_string());
    let topic = env::var("KAFKA_TOPIC").unwrap_or_else(|_| DEFAULT_KAFKA_TOPIC.to_string());

    let consumer: StreamConsumer = ClientConfig::new()
      .set("bootstrap.servers", &bootstrap_servers)
      .set("group.id", &group_id)
      .set("enable.partition.eof", "false")
      .set("session.timeout.ms", "6000")
      .set("enable.auto.commit", "true")
      .set("auto.offset.reset", "earliest")
      .create()
      .context("Failed to create Kafka consumer")?;

    Ok(CustomerCrmService {
      consumer,
      mailer: EmailSender::new()?,
      topic,
    })
  }

  /// Run the consumer loop until the process is stopped.
  pub async fn run(self) -> Result<()> {
    self
      .consumer
      .subscribe(&[&self.topic])
      .context("Failed to subscribe to Kafka topic")?;

    loop {
      let message = self.consumer.recv().await.context("Kafka consumer error")?;
      let payload = message.payload().context("Kafka message missing payload")?;
      let event: CustomerRegisteredEvent =
        serde_json::from_slice(payload).context("Failed to parse customer event JSON")?;

      if let Err(error) = self.mailer.send_activation_email(&event).await {
        eprintln!("{error:#}");
      }
    }
  }
}

/// Initialize the CRM service, mailer, and Kafka connections.
#[tokio::main]
async fn main() -> Result<()> {
  dotenv().ok();

  let service = CustomerCrmService::new()?;
  service.run().await
}
