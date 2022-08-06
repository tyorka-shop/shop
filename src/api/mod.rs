mod product;
mod gql;


pub use gql::{ApiClientConfig, ApiClient, GraphQLClient};
pub use product::{ApiMethods, GetProductError};
