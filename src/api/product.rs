use async_trait::async_trait;
use log::info;
use std::fmt;
use graphql_client::GraphQLQuery;

use crate::api::{ApiClient, GraphQLClient};
use crate::entity::Product;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "src/api/query.graphql",
    response_derives = "Debug"
)]
pub struct ProductQuery;

pub enum GetProductError {
    NotFound,
    NoPrice
}

impl fmt::Display for GetProductError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GetProductError::NotFound => write!(f, "Product not found"),
            GetProductError::NoPrice => write!(f, "Product has no price"),
        }
    }
}

impl Product {
    fn from(response: product_query::ProductQueryProduct) -> Result<Self, GetProductError> {
        let price = match response.price {
            None => return Err(GetProductError::NoPrice),
            Some(price) => price,
        };

        let title = response
            .title
            .and_then(|title| title.ru.or(title.en))
            .unwrap_or_else(|| response.id.clone());

        Ok(Self::new(&response.id, &title, (price * 100_f64) as u32))
    }
}

#[async_trait]
pub trait ApiMethods {
    async fn get_product(&self, id: &str) -> Result<Product, GetProductError>;
}

#[async_trait]
impl ApiMethods for ApiClient {
    async fn get_product(&self, id: &str) -> Result<Product, GetProductError> {
        let request = ProductQuery::build_query(product_query::Variables { id: id.to_string() });
        info!("Try to get product by id: {}", &id);
        match self
            .call_graphql::<product_query::ResponseData, _>(&request)
            .await
        {
            Ok(response) => {
                let product = Product::from(response.product)?;
                info!("Product found: {}", &product.title);
                Ok(product)
            },
            Err(_) => Err(GetProductError::NotFound),
        }
    }
}
