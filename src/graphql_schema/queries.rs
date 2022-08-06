use crate::{
    api::{ApiClient, ApiMethods},
    grant::RoleData,
};
use async_graphql::{Context, Error, Object, Result, SimpleObject};

pub struct Queries;

#[derive(SimpleObject)]
pub struct ProductObject {
    pub id: String,
    pub title: String,
}

#[Object]
impl Queries {
    async fn status(&self) -> Result<&str> {
        Ok("Ok")
    }

    async fn my_role<'a>(&self, ctx: &Context<'a>) -> Result<String> {
        match ctx.data::<RoleData>() {
            Ok(role) => Ok(String::from(role)),
            Err(e) => Err(e),
        }
    }

    async fn product<'a>(&self, ctx: &Context<'a>, id: String) -> Result<String> {
        let api_client = ctx.data::<ApiClient>().unwrap();
        let product = api_client
            .get_product(&id)
            .await
            .map_err(|e| Error::new(format!("{}", e)))?;

        Ok(product.title)
    }
}
