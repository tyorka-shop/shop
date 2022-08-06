mod mutations;
mod queries;

use async_graphql::{EmptySubscription, Schema, SchemaBuilder};
pub use mutations::Mutations;
pub use queries::Queries;

pub type GQLSchema = Schema<Queries, Mutations, EmptySubscription>;

pub fn build_schema() -> SchemaBuilder<Queries, Mutations, EmptySubscription> {
    Schema::build(Queries, Mutations, EmptySubscription)
}
