// Bookstore Mobile App Service
//! Authentication Middleware for Auth-protected Routes.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
  extract::Request,
  http::{header, StatusCode},
  middleware::Next,
  response::Response,
};
use jsonwebtoken::dangerous::insecure_decode;
use serde::Deserialize;

/// JWT claim fields that must be checked.
#[derive(Deserialize)]
struct JwtClaims {
  sub: String,
  exp: u64,
  iss: String,
}

/// Validate JWT claim for expiry and known sub/iss.
pub async fn validate_jwt(request: Request, next: Next) -> Result<Response, StatusCode> {
  const BEARER_PREFIX: &str = "Bearer ";

  const KNOWN_SUB: [&str; 5] = ["starlord", "gamora", "drax", "rocket", "groot"];
  const KNOWN_ISS: [&str; 1] = ["cmu.edu"];

  let auth_header = request
    .headers()
    .get(header::AUTHORIZATION)
    .and_then(|header| header.to_str().ok());

  let token = match auth_header {
    Some(header_value) if header_value.starts_with(BEARER_PREFIX) => {
      &header_value[BEARER_PREFIX.len()..]
    }
    _ => return Err(StatusCode::UNAUTHORIZED),
  };

  let current_timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs();

  match insecure_decode::<JwtClaims>(token) {
    Ok(token_data)
      if KNOWN_SUB.contains(&token_data.claims.sub.as_str())
        && KNOWN_ISS.contains(&token_data.claims.iss.as_str())
        && current_timestamp < token_data.claims.exp =>
    {
      Ok(next.run(request).await)
    }
    _ => Err(StatusCode::UNAUTHORIZED),
  }
}
