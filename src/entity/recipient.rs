use async_graphql::InputObject;
use serde::Deserialize;

#[derive(InputObject, Debug, Deserialize)]
pub struct Recipient {
    pub name: String,
    pub email: String,
}
