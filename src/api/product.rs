use graphql_client::{GraphQLQuery, Response};
use reqwest::{header, RequestBuilder};
use serde::Serialize;
use std::{error::Error};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "src/api/query.graphql",
    response_derives = "Debug"
)]
pub struct ProductQuery;

pub type Product = product_query::ProductQueryProduct;

pub struct ApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl ApiClient {
    pub fn build(url: &str, token: &str) -> Result<Self, Box<dyn Error>> {
        let mut headers = header::HeaderMap::new();
        headers.insert("X-Auth", header::HeaderValue::from_str(token)?);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(ApiClient {
            client,
            base_url: url.into(),
        })
    }

    pub fn call<T: Serialize + ?Sized>(&self, query: &T) -> RequestBuilder {
        self.client
        .post(&format!("{}{}", &self.base_url, "/graphql"))
        .json(&query)
    }

}

pub async fn get_product(client: &ApiClient, id: &str) -> Result<Product, Box<dyn Error>> {
    let request_body = ProductQuery::build_query(product_query::Variables { id: id.to_string() });

    let response = client.call(&request_body)
        .send()
        .await?
        .json::<Response<product_query::ResponseData>>()
        .await?;
    dbg!(&response);
    match response.data {
        Some(data) => Ok(data.product),
        None => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Product not found"))),
    }
}
