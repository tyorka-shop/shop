use crate::{
    entity::{Order, Store},
    grant::RoleData,
};
use async_graphql::{Context, Object, Result};
use sea_orm::DatabaseConnection;

use super::orders::Orders;

pub struct Queries;

#[Object]
impl Queries {
    async fn status(&self) -> Result<String> {
        Ok("Ok".into())
    }

    async fn my_role<'a>(&self, ctx: &Context<'a>) -> Result<String> {
        let result = ctx.data::<RoleData>().unwrap();
        Ok(result.into())
    }

    #[graphql(guard = "RoleData::admin()")]
    async fn orders<'a>(&self, ctx: &Context<'a>) -> Result<Orders> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let list = Order::find(db).await;
        Ok(Orders { list })
    }

    #[graphql(guard = "RoleData::admin()")]
    async fn order<'a>(&self, ctx: &Context<'a>, id: String) -> Result<Order> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let order = Order::find_one(db, &id).await.unwrap();
        Ok(order)
    }
}
