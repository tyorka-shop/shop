mod mutations;
mod queries;
mod recipient_input;
mod order_input;
mod product_input;
mod orders;

use async_graphql::{EmptySubscription, Schema, SchemaBuilder};
pub use mutations::Mutations;
pub use queries::Queries;

pub use recipient_input::RecipientInput;

pub type GQLSchema = Schema<Queries, Mutations, EmptySubscription>;

pub fn build_schema() -> SchemaBuilder<Queries, Mutations, EmptySubscription> {
    Schema::build(Queries, Mutations, EmptySubscription)
}
