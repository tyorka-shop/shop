use super::id::ID;

pub struct Product {
  pub id: ID,
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