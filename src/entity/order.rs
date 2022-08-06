use super::{Recipient};
use async_graphql::InputObject;

#[derive(InputObject, Debug)]
pub struct Order {
  pub recipient: Recipient,
  /// list of product ids
  pub cart: Vec<String>,
}
