use async_graphql::SimpleObject;
use entity::cart_item;

#[derive(SimpleObject, Debug)]
pub struct Product {
  pub id: String,
  pub title: String,
  pub count: u32,
  pub price: u32,
}

impl Product {
  pub fn new(id: &str, title: &str, price: u32) -> Self {
    Self {
      id: id.into(),
      title: title.to_string(),
      count: 1,
      price,
    }
  }
}

impl From<cart_item::Model> for Product {
  fn from(item: cart_item::Model) -> Self {
    Self {
      id: item.product_id,
      title: item.title,
      count: item.count,
      price: item.price,
    }
  }
}