use async_graphql::InputObject;

use super::{recipient_input::RecipientInput, product_input::ProductInput};

#[derive(InputObject)]
pub struct OrderInput {
    pub recipient: RecipientInput,
    pub cart: Vec<ProductInput>,
    pub captcha: String,
}
