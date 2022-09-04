use async_graphql::SimpleObject;

#[derive(SimpleObject, Debug)]
pub struct CartItem {
  pub id: String,
  pub count: u32,
  pub price: u32,
}
