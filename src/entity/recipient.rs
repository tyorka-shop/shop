use async_graphql::SimpleObject;
use serde::Deserialize;

use crate::graphql_schema::RecipientInput;

#[derive(SimpleObject, Debug, Deserialize)]
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