use async_graphql::InputObject;

#[derive(InputObject)]
pub struct RecipientInput {
    pub name: String,
    pub email: String,
}
