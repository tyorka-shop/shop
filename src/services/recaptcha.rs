use async_trait::async_trait;
use config::RecaptchaConfig;
use log::{error, info};
use sea_orm::strum::Display;
use serde::Deserialize;

pub struct Recaptcha {
    config: RecaptchaConfig,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    success: bool,
}

#[derive(Debug, Display)]
pub enum RecaptchaError {
    HttpError,
    InvalidResponse,
    WrongCaptcha,
}

#[async_trait]
pub trait RecaptchaMethods {
    fn new(config: &RecaptchaConfig) -> Self;
    async fn verify(&self, value: &str) -> Result<(), RecaptchaError>;
}

#[async_trait]
impl RecaptchaMethods for Recaptcha {
    fn new(config: &RecaptchaConfig) -> Self {
        Recaptcha {
            config: config.clone(),
        }
    }

    async fn verify(&self, value: &str) -> Result<(), RecaptchaError> {
        if !self.config.enabled {
            info!("Recaptcha is disabled");
            return Ok(());
        }
        info!("Verifying recaptcha with value: {}", value);
        let response = reqwest::Client::new()
            .post("https://www.google.com/recaptcha/api/siteverify")
            .form(&[
                ("secret", &self.config.key),
                ("response", &value.to_string()),
            ])
            .send()
            .await
            .map_err(|e| {
                error!("Can not verify captcha {:?}", e);
                RecaptchaError::HttpError
            })?
            .json::<Response>()
            .await
            .map_err(|e| {
                error!("Invalid captcha response: {:?}", e);
                RecaptchaError::InvalidResponse
            })?;

        info!("Response: {:?}", response);
        match response.success {
            true => Ok(()),
            false => {
                error!("Wrong captcha");
                Err(RecaptchaError::WrongCaptcha)
            }
        }
    }
}
