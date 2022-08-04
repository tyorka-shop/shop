#![allow(clippy::all, warnings)]
pub struct ProductQuery;
pub mod product_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ProductQuery";
    pub const QUERY : & str = "query ProductQuery($id: ID!) {\n  product(id: $id) {\n    id\n    title {\n      ru\n    }\n    cover {\n      src\n    }\n    price\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables {
        pub id: ID,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub product: ProductQueryProduct,
    }
    #[derive(Deserialize)]
    pub struct ProductQueryProduct {
        pub id: ID,
        pub title: Option<ProductQueryProductTitle>,
        pub cover: Option<ProductQueryProductCover>,
        pub price: Option<Float>,
    }
    #[derive(Deserialize)]
    pub struct ProductQueryProductTitle {
        pub ru: Option<String>,
    }
    #[derive(Deserialize)]
    pub struct ProductQueryProductCover {
        pub src: String,
    }
}
impl graphql_client::GraphQLQuery for ProductQuery {
    type Variables = product_query::Variables;
    type ResponseData = product_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: product_query::QUERY,
            operation_name: product_query::OPERATION_NAME,
        }
    }
}
