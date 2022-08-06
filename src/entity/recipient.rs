use async_graphql::InputObject;

#[derive(InputObject, Debug)]
pub struct Recipient {
  pub name: String,
  pub email: String
}
