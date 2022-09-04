use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TgBotConfig {
    pub enabled: bool,
    pub token: String,
    pub chats: Vec<String>,
}

impl Default for TgBotConfig {
  fn default() -> Self {
      Self {
          enabled: false,
          token: "xxx".into(),
          chats: vec![],
      }
  }
}
