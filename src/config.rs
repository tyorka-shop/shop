use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiClient {
    pub url: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub port: String,
    pub secret: String,

    pub api_client: ApiClient,
}

pub fn make_secret_key() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(256)
        .map(char::from)
        .collect()
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            port: "3001".into(),
            secret: make_secret_key(),
            api_client: ApiClient {
                url: "http://localhost:3000".into(),
                token: "xxx".into(),
            },
        }
    }
}

pub fn load() -> Config {
    confy::load::<Config>("tyorka-shop").unwrap()
}
