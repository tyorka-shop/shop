use async_graphql::SimpleObject;

use crate::graphql_schema::RecipientInput;

#[derive(SimpleObject, Debug)]
pub struct Recipient {
    pub name: String,
    pub email: String,
}


impl From<RecipientInput> for Recipient {
    fn from(input: RecipientInput) -> Self {
        Self {
            name: input.name,
            email: input.email,
        }
    }
}