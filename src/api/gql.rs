use async_trait::async_trait;
use config::ApiClientConfig;
use graphql_client::Response;
use log::{debug, error, info};
use reqwest::header;
use serde::{de::DeserializeOwned, Serialize};
use std::{error::Error, fmt::Debug};

use crate::cache;

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
    BadResponse,
    ReqwestError,
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
        let response = match cache::read(&query) {
            Some(response) => {
                info!("Get response from cache");
                response
            },
            None => self
                .client
                .post(&format!("{}/graphql", &self.base_url))
                .json(&query)
                .send()
                .await
                .map_err(|e| {
                    error!("{}", e);
                    GQLError::NoData
                })?
                .text()
                .await
                .map_err(|e| {
                    error!("{}", e);
                    GQLError::BadResponse
                })?,
        };

        cache::write(&query, &response, 60);

        let inner = serde_json::from_str::<Response<R>>(&response).map_err(|e| {
            error!("{}", e);
            GQLError::BadResponse
        })?;

        debug!("{:?}", &inner);

        match inner.data {
            Some(data) => Ok(data),
            None => {
                error!("{:?}", &inner.errors);
                Err(GQLError::ReqwestError)
            }
        }
    }
}
