use serde::{Serialize, Deserialize};

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
