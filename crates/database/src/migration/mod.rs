use sea_orm_migration::prelude::*;

mod m20260811_000001_create_admin_tables;
mod m20260811_000002_create_events;
mod m20260811_000003_create_entries;
mod m20260811_000004_add_entry_images;
mod m20260811_000005_create_votes;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260811_000001_create_admin_tables::Migration),
            Box::new(m20260811_000002_create_events::Migration),
            Box::new(m20260811_000003_create_entries::Migration),
            Box::new(m20260811_000004_add_entry_images::Migration),
            Box::new(m20260811_000005_create_votes::Migration),
        ]
    }
}
