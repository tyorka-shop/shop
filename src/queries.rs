use async_graphql::{Context, Error, Object, Result, SimpleObject};
use crate::{
    api::{get_product, ApiClient},
    grant::RoleData,
};

pub struct Query;

#[derive(SimpleObject)]
pub struct ProductObject {
  pub id: String,
  pub title: String,
}

#[Object]
impl Query {
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
        let product = get_product(&api_client, &id)
            .await
            .map_err(|e| Error::new(format!("{}", e)))?;
        match product.title {
            Some(title) => Ok(title.ru.unwrap_or_default()),
            None => Err(Error::from(format!("Product not found"))),
        }
    }
}
