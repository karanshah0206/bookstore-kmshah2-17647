// Customer CRM Service
//! Gmail SMTP mailer for activation emails.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use std::env;

use anyhow::{Context, Result};
use lettre::{
  AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
  transport::smtp::authentication::Credentials,
};

use crate::dto::customer::CustomerRegisteredEvent;

const DEFAULT_SMTP_USERNAME: &str = "karanmshah02@gmail.com";
const DEFAULT_SMTP_APP_PASSWORD: &str = "qsid ewio ugaa drrb";
const DEFAULT_SMTP_FROM_ADDRESS: &str = "karanmshah02@gmail.com";

/// Email sender for customer activation messages.
pub struct EmailSender {
  mailer: AsyncSmtpTransport<Tokio1Executor>,
  from_address: String,
}

impl EmailSender {
  /// Build an SMTP mailer configured for Gmail.
  pub fn new() -> Result<Self> {
    let username = env::var("SMTP_USERNAME").unwrap_or_else(|_| DEFAULT_SMTP_USERNAME.to_string());
    let app_password =
      env::var("SMTP_APP_PASSWORD").unwrap_or_else(|_| DEFAULT_SMTP_APP_PASSWORD.to_string());
    let from_address =
      env::var("SMTP_FROM_ADDRESS").unwrap_or_else(|_| DEFAULT_SMTP_FROM_ADDRESS.to_string());

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
      .context("Failed to configure Gmail SMTP transport")?
      .credentials(Credentials::new(username, app_password))
      .build();

    Ok(EmailSender {
      mailer,
      from_address,
    })
  }

  /// Send the activation email to the newly registered customer.
  pub async fn send_activation_email(&self, customer: &CustomerRegisteredEvent) -> Result<()> {
    let email = Message::builder()
      .from(self.from_address.parse().context("Invalid sender email address")?)
      .to(customer
        .user_id
        .parse()
        .context("Invalid customer recipient email address")?)
      .subject("Activate your book store account")
      .body(format!(
        "Dear {},\nWelcome to the Book store created by kmshah2.\nExceptionally this time we won’t ask you to click a link to activate your account.",
        customer.name
      ))
      .context("Failed to build activation email message")?;

    self
      .mailer
      .send(email)
      .await
      .context("Failed to send activation email")?;

    Ok(())
  }
}
