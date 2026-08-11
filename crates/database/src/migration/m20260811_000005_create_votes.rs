use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Votes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Votes::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Votes::EventId).integer().not_null())
                    .col(ColumnDef::new(Votes::VoterHash).string().not_null())
                    .col(ColumnDef::new(Votes::EntryId).integer().not_null())
                    .col(ColumnDef::new(Votes::CreatedAt).big_integer().not_null())
                    .col(ColumnDef::new(Votes::UpdatedAt).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-votes-event")
                            .from(Votes::Table, Votes::EventId)
                            .to(Events::Table, Events::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-votes-entry")
                            .from(Votes::Table, Votes::EntryId)
                            .to(Entries::Table, Entries::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-votes-event-voter")
                    .table(Votes::Table)
                    .col(Votes::EventId)
                    .col(Votes::VoterHash)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-votes-event-entry")
                    .table(Votes::Table)
                    .col(Votes::EventId)
                    .col(Votes::EntryId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Votes::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Votes {
    Table,
    Id,
    EventId,
    VoterHash,
    EntryId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Events {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Entries {
    Table,
    Id,
}
