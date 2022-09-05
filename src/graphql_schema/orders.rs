use async_graphql::SimpleObject;
use crate::entity::Order;

#[derive(SimpleObject)]
pub struct Orders {
  pub list: Vec<Order>,
}