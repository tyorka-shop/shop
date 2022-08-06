use async_trait::async_trait;
use graphql_client::Response;
use log::{debug, error};
use reqwest::header;
use serde::{de::DeserializeOwned, Serialize, Deserialize};
use std::{error::Error, fmt::Debug};

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

#[derive(Clone, Debug)]
pub struct ApiClient {
    client: reqwest::Client,
    base_url: String,
}

#[async_trait]
pub trait GraphQLClient: Sized {
    fn new(config: &ApiClientConfig) -> Result<Self, Box<dyn Error>>;
    async fn call_graphql<R: DeserializeOwned + Debug, T: Serialize + ?Sized + Sync>(
        &self,
        query: &T,
    ) -> Result<R, GQLError>;
}

pub enum GQLError {
    NoData,
}

#[async_trait]
impl GraphQLClient for ApiClient {
    fn new(config: &ApiClientConfig) -> Result<Self, Box<dyn Error>> {
        let mut headers = header::HeaderMap::new();
        headers.insert("X-Auth", header::HeaderValue::from_str(&config.token)?);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(ApiClient {
            client,
            base_url: config.url.to_string(),
        })
    }

    async fn call_graphql<R, T>(&self, query: &T) -> Result<R, GQLError>
    where
        R: DeserializeOwned + Debug,
        T: Serialize + ?Sized + Sync,
    {
        let response = self
            .client
            .post(&format!("{}{}", &self.base_url, "/graphql"))
            .json(&query)
            .send()
            .await
            .map_err(|e| {
                error!("{}", e);
                GQLError::NoData
            })?;

        let inner = response.json::<Response<R>>().await.map_err(|e| {
            error!("{}", e);
            GQLError::NoData
        })?;

        debug!("{:?}", &inner);

        match inner.data {
            Some(data) => Ok(data),
            None => {
                error!("{:?}", &inner.errors);
                Err(GQLError::NoData)
            }
        }
    }
}
