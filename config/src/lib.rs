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
    pub database_uri: String,
    pub cors_allowed_origins: Vec<String>,

    pub api_client: ApiClientConfig,
    pub tg_client: TgBotConfig,
    pub recaptcha: RecaptchaConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: "3001".into(),
            secret: make_secret_key(),
            cors_allowed_origins: vec!["http://localhost:3000".into()],
            database_uri: "sqlite:./store/data.db".into(),
            api_client: Default::default(),
            tg_client: Default::default(),
            recaptcha: Default::default(),
        }
    }
}

pub fn load(name: &str) -> Config {
    confy::load::<Config>(name).unwrap()
}
