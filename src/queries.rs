use async_graphql::{Object, Result, Context};

use crate::grant::RoleData;

pub struct Query;

#[Object]
impl Query {
    async fn status(&self) -> Result<&str> {
        Ok("Ok")
    }

    async fn my_role<'a>(&self, ctx: &Context<'a>) -> Result<String> {
      match ctx.data::<RoleData>() {
        Ok(role) => Ok(String::from(role)),
        Err(e) => Err(e)
      }
    }
}
