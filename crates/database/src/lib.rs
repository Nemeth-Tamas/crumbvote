pub mod entity;
mod migration;

use sea_orm::{ConnectOptions, Database, DbErr};
use sea_orm_migration::MigratorTrait;
use std::time::Duration;

pub use sea_orm::DatabaseConnection;

pub async fn connect(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let max_connections = if database_url.contains(":memory:") {
        1
    } else {
        5
    };

    let mut options = ConnectOptions::new(database_url.to_owned());

    options
        .max_connections(max_connections)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(5))
        .sqlx_logging(false);

    let database = Database::connect(options).await?;

    migration::Migrator::up(&database, None).await?;

    Ok(database)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm_migration::SchemaManager;

    #[tokio::test]
    async fn fresh_database_runs_initial_migration() {
        let database = connect("sqlite::memory:")
            .await
            .expect("in-memory database should connect");

        let schema = SchemaManager::new(&database);

        assert!(
            schema
                .has_table("admin_credentials")
                .await
                .expect("admin_credentials schema check should succeed")
        );

        assert!(
            schema
                .has_table("admin_sessions")
                .await
                .expect("admin_sessions schema check should succeed")
        );
    }
}
