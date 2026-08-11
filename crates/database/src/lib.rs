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

pub async fn admin_password_hash(database: &DatabaseConnection) -> Result<Option<String>, DbErr> {
    Ok(
        entity::admin_credential::Entity::find_by_id(ADMIN_CREDENTIAL_ID)
            .one(database)
            .await?
            .map(|credential| credential.password_hash),
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

pub async fn create_admin_session(
    database: &DatabaseConnection,
    token_hash: String,
    ttl_seconds: i64,
) -> Result<(), DbErr> {
    let now = unix_timestamp()?;

    let expires_at = now
        .checked_add(ttl_seconds)
        .ok_or_else(|| DbErr::Custom("admin session expiration overflow".to_owned()))?;

    entity::admin_session::ActiveModel {
        token_hash: Set(token_hash),
        created_at: Set(now),
        expires_at: Set(expires_at),
        last_seen_at: Set(now),
    }
    .insert(database)
    .await?;

    Ok(())
}

pub async fn admin_session_is_valid(
    database: &DatabaseConnection,
    token_hash: &str,
) -> Result<bool, DbErr> {
    let session = entity::admin_session::Entity::find_by_id(token_hash.to_owned())
        .one(database)
        .await?;

    let Some(session) = session else {
        return Ok(false);
    };

    let now = unix_timestamp()?;

    if session.expires_at <= now {
        entity::admin_session::Entity::delete_by_id(token_hash.to_owned())
            .exec(database)
            .await?;

        return Ok(false);
    }

    let mut active_session: entity::admin_session::ActiveModel = session.into();

    active_session.last_seen_at = Set(now);

    active_session.update(database).await?;

    Ok(true)
}

pub async fn delete_admin_session(
    database: &DatabaseConnection,
    token_hash: &str,
) -> Result<(), DbErr> {
    entity::admin_session::Entity::delete_by_id(token_hash.to_owned())
        .exec(database)
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

        assert_eq!(
            admin_password_hash(&database)
                .await
                .expect("admin password hash lookup should succeed")
                .as_deref(),
            Some("test-password-hash")
        );

        create_admin_session(&database, "test-session-hash".to_owned(), 3600)
            .await
            .expect("admin session should be created");

        assert!(
            admin_session_is_valid(&database, "test-session-hash",)
                .await
                .expect("admin session validation should succeed")
        );

        delete_admin_session(&database, "test-session-hash")
            .await
            .expect("admin session should be deleted");

        assert!(
            !admin_session_is_valid(&database, "test-session-hash",)
                .await
                .expect("deleted admin session should be invalid")
        );
    }
}
