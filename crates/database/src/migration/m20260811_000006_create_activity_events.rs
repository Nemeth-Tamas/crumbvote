use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ActivityEvents::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ActivityEvents::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ActivityEvents::EventId).integer().not_null())
                    .col(ColumnDef::new(ActivityEvents::EntryId).integer().not_null())
                    .col(
                        ColumnDef::new(ActivityEvents::VoterHash)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ActivityEvents::Kind).string().not_null())
                    .col(
                        ColumnDef::new(ActivityEvents::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-activity-events-event")
                            .from(ActivityEvents::Table, ActivityEvents::EventId)
                            .to(Events::Table, Events::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-activity-events-entry")
                            .from(ActivityEvents::Table, ActivityEvents::EntryId)
                            .to(Entries::Table, Entries::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-activity-event-kind-created")
                    .table(ActivityEvents::Table)
                    .col(ActivityEvents::EventId)
                    .col(ActivityEvents::Kind)
                    .col(ActivityEvents::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-activity-event-voter-created")
                    .table(ActivityEvents::Table)
                    .col(ActivityEvents::EventId)
                    .col(ActivityEvents::VoterHash)
                    .col(ActivityEvents::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-activity-event-entry-kind")
                    .table(ActivityEvents::Table)
                    .col(ActivityEvents::EventId)
                    .col(ActivityEvents::EntryId)
                    .col(ActivityEvents::Kind)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ActivityEvents::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ActivityEvents {
    Table,
    Id,
    EventId,
    EntryId,
    VoterHash,
    Kind,
    CreatedAt,
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
