use crate::{
    api::ApiClientConfig,
    services::{RecaptchaConfig, TgBotConfig},
};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub port: String,
    pub secret: String,

    pub api_client: ApiClientConfig,
    pub tg_client: TgBotConfig,
    pub recaptcha: RecaptchaConfig,
}

pub fn make_secret_key() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(256)
        .map(char::from)
        .collect()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: "3001".into(),
            secret: make_secret_key(),
            api_client: Default::default(),
            tg_client: Default::default(),
            recaptcha: Default::default(),
        }
    }
}

pub fn load() -> Config {
    confy::load::<Config>("tyorka-shop").unwrap()
}
