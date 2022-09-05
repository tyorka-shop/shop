mod product;
mod gql;


pub use gql::{ApiClient, GraphQLClient};
pub use product::{ApiMethods, GetProductError, Product};
