pub mod entity;
mod migration;

use sea_orm::{ActiveModelTrait, ConnectOptions, Database, DbErr, EntityTrait, Set};
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

const ADMIN_CREDENTIAL_ID: i32 = 1;

pub async fn admin_is_configured(database: &DatabaseConnection) -> Result<bool, DbErr> {
    Ok(
        entity::admin_credential::Entity::find_by_id(ADMIN_CREDENTIAL_ID)
            .one(database)
            .await?
            .is_some(),
    )
}

pub async fn create_admin_credential(
    database: &DatabaseConnection,
    password_hash: String,
) -> Result<(), DbErr> {
    let now = unix_timestamp()?;

    entity::admin_credential::ActiveModel {
        id: Set(ADMIN_CREDENTIAL_ID),
        password_hash: Set(password_hash),
        created_at: Set(now),
        updated_at: Set(now),
    }
    .insert(database)
    .await?;

    Ok(())
}

fn unix_timestamp() -> Result<i64, DbErr> {
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|error| DbErr::Custom(format!("system clock is before the Unix epoch: {error}")))?
        .as_secs();

    i64::try_from(seconds)
        .map_err(|_| DbErr::Custom("current Unix timestamp does not fit in i64".to_owned()))
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

        assert!(
            !admin_is_configured(&database)
                .await
                .expect("admin configuration check should succeed")
        );

        create_admin_credential(&database, "test-password-hash".to_owned())
            .await
            .expect("admin credential should be created");

        assert!(
            admin_is_configured(&database)
                .await
                .expect("admin configuration check should succeed")
        );
    }
}
