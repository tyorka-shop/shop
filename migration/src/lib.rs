pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_cart_item;
mod m20220903_141616_mail_sendings;
mod m20220903_144201_order;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_cart_item::Migration),
            Box::new(m20220903_141616_mail_sendings::Migration),
            Box::new(m20220903_144201_order::Migration),
        ]
    }
}
