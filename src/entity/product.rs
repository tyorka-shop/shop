use async_graphql::SimpleObject;

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
