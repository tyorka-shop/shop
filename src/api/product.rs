use async_trait::async_trait;
use graphql_client::GraphQLQuery;
use std::{error::Error, io};

use crate::api::client::{ApiClient, ApiClientExt};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "src/api/query.graphql",
    response_derives = "Debug"
)]
pub struct ProductQuery;

pub type Product = product_query::ProductQueryProduct;

#[async_trait]
pub trait ApiMethods {
    async fn get_product(&self, id: &str) -> Result<Product, Box<dyn Error>>;
}

#[async_trait]
impl ApiMethods for ApiClient {
    async fn get_product(&self, id: &str) -> Result<Product, Box<dyn Error>> {
        let request = ProductQuery::build_query(product_query::Variables { id: id.to_string() });

        match self
            .call_graphql::<product_query::ResponseData, _>(&request)
            .await
        {
            Ok(response) => Ok(response.product),
            Err(_) => Err(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                "Product not found",
            ))),
        }
    }
}
