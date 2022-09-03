use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Recipient::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Recipient::Id)
                            .string()
                            .not_null()
                            .uuid()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Recipient::Name).string().not_null())
                    .col(ColumnDef::new(Recipient::Email).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Recipient::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Recipient {
    Table,
    
    Id,
    Name,
    Email,
}
