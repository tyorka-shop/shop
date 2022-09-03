use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiClientConfig {
    pub url: String,
    pub token: String,
}

impl Default for ApiClientConfig {
    fn default() -> Self {
        Self {
            url: "http://localhost:3000".into(),
            token: "xxx".into(),
        }
    }
}
