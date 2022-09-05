use async_graphql::SimpleObject;
use entity::cart_item;

use super::Product;

#[derive(SimpleObject, Debug)]
pub struct CartItem {
  pub product: Product,
  pub count: u32,
  pub price: u32,
}

impl From<cart_item::Model> for CartItem {
  fn from(item: cart_item::Model) -> Self {
    Self {
      product: Product::new(&item.product_id, &item.title),
      count: item.count,
      price: item.price,
    }
  }
}

impl From<crate::api::Product> for CartItem {
  fn from(product: crate::api::Product) -> Self {
    Self {
      product: Product::new(&product.id, &product.title),
      count: 1,
      price: product.price,
    }
  }
}