use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TgBotConfig {
    pub chats: Vec<String>,
    pub enabled: bool,
    pub token: String,
}

impl Default for TgBotConfig {
  fn default() -> Self {
      Self {
          chats: vec![],
          enabled: false,
          token: "xxx".into(),
      }
  }
}
