use async_trait::async_trait;
use config::TgBotConfig;
use log::info;
use serde::Serialize;
use std::fmt;

pub struct TgBot {
    config: TgBotConfig,
}

pub enum TgError {
    HttpError(reqwest::Error),
}

impl fmt::Display for TgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::HttpError(e) => write!(f, "{}", e),
        }
    }
}

#[derive(Serialize)]
struct TgMessage {
    chat_id: String,
    text: String,
}

#[async_trait]
pub trait TgBotExt {
    fn new(config: &TgBotConfig) -> Self;
    async fn send_message(&self, chat_id: &str, message: &str) -> Result<(), TgError>;
    async fn send_messages(&self, message: &str) -> Result<(), TgError>;
}

#[async_trait]
impl TgBotExt for TgBot {
    fn new(config: &TgBotConfig) -> Self {
        TgBot {
            config: config.clone(),
        }
    }

    async fn send_message(&self, chat_id: &str, message: &str) -> Result<(), TgError> {
        if !self.config.enabled {
            info!("Tg bot is disabled");
            return Ok(());
        }

        let body = TgMessage {
            chat_id: chat_id.into(),
            text: message.to_string(),
        };

        info!("Sending message to tg chat: {}", message);

        reqwest::Client::new()
            .post(format!(
                "https://api.telegram.org/bot{}/sendMessage",
                self.config.token
            ))
            .json(&body)
            .send()
            .await
            .map_err(|e| TgError::HttpError(e))?;
        Ok(())
    }

    async fn send_messages(&self, message: &str) -> Result<(), TgError> {
        for chat_id in &self.config.chats {
            self.send_message(chat_id, message).await?;
        }
        Ok(())
    }
}
