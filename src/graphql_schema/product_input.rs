use async_graphql::InputObject;

#[derive(InputObject)]
pub struct ProductInput {
    pub id: String,
    pub count: Option<u32>,
}
