use random::make_secret_key;
use serde::{Deserialize, Serialize};

mod random;
mod recaptcha;
mod tgbot;
mod api_client;

pub use recaptcha::RecaptchaConfig;
pub use tgbot::TgBotConfig;
pub use api_client::ApiClientConfig;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub port: String,
    pub secret: String,

    pub api_client: ApiClientConfig,
    pub tg_client: TgBotConfig,
    pub recaptcha: RecaptchaConfig,

    pub database_uri: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: "3001".into(),
            secret: make_secret_key(),
            api_client: Default::default(),
            tg_client: Default::default(),
            recaptcha: Default::default(),
            database_uri: "sqlite:./store/data.db".into(),
        }
    }
}

pub fn load(name: &str) -> Config {
    confy::load::<Config>(name).unwrap()
}
