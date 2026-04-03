// Bookstore Books Service
//! Data Transfer Objects for Interfacing with Gemini LLM.
// Author: Karan Manoj Shah <kmshah2@cs.cmu.edu>

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GeminiRequest {
  pub contents: Vec<GeminiContent>,
  #[serde(rename = "generationConfig")]
  pub generation_config: GeminiGenerationConfig,
}

#[derive(Serialize)]
pub struct GeminiContent {
  pub parts: Vec<GeminiPart>,
}

#[derive(Serialize)]
pub struct GeminiPart {
  pub text: String,
}

#[derive(Serialize)]
pub struct GeminiGenerationConfig {
  pub temperature: f32,
  #[serde(rename = "maxOutputTokens")]
  pub max_output_tokens: u32,
}

#[derive(Deserialize)]
pub struct GeminiResponse {
  pub candidates: Option<Vec<GeminiCandidate>>,
}

#[derive(Deserialize)]
pub struct GeminiCandidate {
  pub content: Option<GeminiResponseContent>,
}

#[derive(Deserialize)]
pub struct GeminiResponseContent {
  pub parts: Option<Vec<GeminiResponsePart>>,
}

#[derive(Deserialize)]
pub struct GeminiResponsePart {
  pub text: Option<String>,
}
