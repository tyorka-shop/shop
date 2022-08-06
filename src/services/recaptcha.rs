use async_trait::async_trait;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecaptchaConfig {
  pub enabled: bool,
  pub key: String
}

impl Default for RecaptchaConfig {
  fn default() -> Self {
    Self {
      enabled: false,
      key: "xxx".into()
    }
  }
}

pub struct Recaptcha {
  config: RecaptchaConfig
}

#[derive(Deserialize, Debug)]
pub struct Response {
  success: bool
}

pub enum RecaptchaError {
  HttpError(reqwest::Error),
  InvalidResponse(reqwest::Error)
}

#[async_trait]
pub trait RecaptchaMethods {
  fn new(config: &RecaptchaConfig) -> Self;
  async fn verify(&self, value: &str) -> Result<bool, RecaptchaError>;
}

#[async_trait]
impl RecaptchaMethods for Recaptcha {
  fn new(config: &RecaptchaConfig) -> Self {
    Recaptcha {
      config: config.clone()
    }
  }

  async fn verify(&self, value: &str) -> Result<bool, RecaptchaError> {
    if !self.config.enabled {
      info!("Recaptcha is disabled");
      return Ok(true);
    }
    info!("Verifying recaptcha with value: {}", value);
    let client = reqwest::Client::new();
    let response = match client.post("https://www.google.com/recaptcha/api/siteverify")
      .form(&[
        ("secret", &self.config.key),
        ("response", &value.to_string())
      ])
      .send()
      .await {
        Ok(response) => response,
        Err(e) => return Err(RecaptchaError::HttpError(e)) 
      };
      
    let response = match response.json::<Response>().await {
      Ok(body) => body,
      Err(e) => return Err(RecaptchaError::InvalidResponse(e))
    };

    info!("Response: {:?}", response);
    Ok(response.success)
  }
}