use async_graphql::SimpleObject;
use entity::cart_item;

#[derive(SimpleObject, Debug)]
pub struct Product {
    pub id: String,
    pub title: String,
}

impl Product {
    pub fn new(id: &str, title: &str) -> Self {
        Self {
            id: id.into(),
            title: title.to_string(),
        }
    }
}

impl From<cart_item::Model> for Product {
    fn from(item: cart_item::Model) -> Self {
        Self {
            id: item.product_id,
            title: item.title,
        }
    }
}
