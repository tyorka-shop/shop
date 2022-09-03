use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MailSending::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MailSending::Id)
                            .string()
                            .not_null()
                            .uuid()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MailSending::Date).string().not_null())
                    .col(ColumnDef::new(MailSending::OrderId).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MailSending::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum MailSending {
    Table,
    Id,
    Date,
    OrderId,
}
