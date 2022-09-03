use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CartItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CartItem::Id)
                            .string()
                            .not_null()
                            .uuid()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CartItem::Count).integer().not_null())
                    .col(ColumnDef::new(CartItem::Price).integer().not_null())
                    .col(ColumnDef::new(CartItem::OrderId).string().not_null())
                    .col(ColumnDef::new(CartItem::ProductId).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CartItem::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum CartItem {
    Table,
    
    Id,
    Count,
    Price,
    OrderId,
    ProductId,
}
