use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Entries::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Entries::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Entries::EventId).integer().not_null())
                    .col(ColumnDef::new(Entries::Number).integer().not_null())
                    .col(ColumnDef::new(Entries::Name).string().not_null())
                    .col(ColumnDef::new(Entries::Description).text().null())
                    .col(ColumnDef::new(Entries::CreatedAt).big_integer().not_null())
                    .col(ColumnDef::new(Entries::UpdatedAt).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-entries-event")
                            .from(Entries::Table, Entries::EventId)
                            .to(Events::Table, Events::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-entries-event-number")
                    .table(Entries::Table)
                    .col(Entries::EventId)
                    .col(Entries::Number)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entries::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Entries {
    Table,
    Id,
    EventId,
    Number,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Events {
    Table,
    Id,
}
