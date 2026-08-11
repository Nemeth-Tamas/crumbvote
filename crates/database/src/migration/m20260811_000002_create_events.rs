use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Events::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Events::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Events::Slug)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Events::Title).string().not_null())
                    .col(ColumnDef::new(Events::Description).text().null())
                    .col(ColumnDef::new(Events::Status).string().not_null())
                    .col(ColumnDef::new(Events::ResultsPublic).boolean().not_null())
                    .col(ColumnDef::new(Events::CreatedAt).big_integer().not_null())
                    .col(ColumnDef::new(Events::UpdatedAt).big_integer().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Events::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Events {
    Table,
    Id,
    Slug,
    Title,
    Description,
    Status,
    ResultsPublic,
    CreatedAt,
    UpdatedAt,
}
