pub mod entity;
mod migration;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectOptions, Database, DbErr, EntityTrait, NotSet,
    QueryFilter, QueryOrder, Set,
};
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

pub use entity::event::Model as EventModel;

pub async fn event_slug_exists(database: &DatabaseConnection, slug: &str) -> Result<bool, DbErr> {
    Ok(entity::event::Entity::find()
        .filter(entity::event::Column::Slug.eq(slug))
        .one(database)
        .await?
        .is_some())
}

pub async fn create_event(
    database: &DatabaseConnection,
    slug: String,
    title: String,
    description: Option<String>,
) -> Result<EventModel, DbErr> {
    let now = unix_timestamp()?;

    entity::event::ActiveModel {
        id: NotSet,
        slug: Set(slug),
        title: Set(title),
        description: Set(description),
        status: Set("draft".to_owned()),
        results_public: Set(false),
        created_at: Set(now),
        updated_at: Set(now),
    }
    .insert(database)
    .await
}

pub async fn list_events(database: &DatabaseConnection) -> Result<Vec<EventModel>, DbErr> {
    entity::event::Entity::find()
        .order_by_desc(entity::event::Column::CreatedAt)
        .all(database)
        .await
}

pub async fn event_by_id(
    database: &DatabaseConnection,
    event_id: i32,
) -> Result<Option<EventModel>, DbErr> {
    entity::event::Entity::find_by_id(event_id)
        .one(database)
        .await
}

pub async fn event_by_slug(
    database: &DatabaseConnection,
    slug: &str,
) -> Result<Option<EventModel>, DbErr> {
    entity::event::Entity::find()
        .filter(entity::event::Column::Slug.eq(slug))
        .one(database)
        .await
}

pub async fn update_event(
    database: &DatabaseConnection,
    event_id: i32,
    title: String,
    description: Option<String>,
    status: String,
    results_public: bool,
) -> Result<Option<EventModel>, DbErr> {
    let Some(event) = entity::event::Entity::find_by_id(event_id)
        .one(database)
        .await?
    else {
        return Ok(None);
    };

    let now = unix_timestamp()?;

    let mut active_event: entity::event::ActiveModel = event.into();

    active_event.title = Set(title);
    active_event.description = Set(description);
    active_event.status = Set(status);
    active_event.results_public = Set(results_public);
    active_event.updated_at = Set(now);

    Ok(Some(active_event.update(database).await?))
}

pub use entity::entry::Model as EntryModel;

pub async fn list_entries(
    database: &DatabaseConnection,
    event_id: i32,
) -> Result<Vec<EntryModel>, DbErr> {
    entity::entry::Entity::find()
        .filter(entity::entry::Column::EventId.eq(event_id))
        .order_by_asc(entity::entry::Column::Number)
        .all(database)
        .await
}

pub async fn create_entry(
    database: &DatabaseConnection,
    event_id: i32,
    name: String,
    description: Option<String>,
) -> Result<EntryModel, DbErr> {
    let last_entry = entity::entry::Entity::find()
        .filter(entity::entry::Column::EventId.eq(event_id))
        .order_by_desc(entity::entry::Column::Number)
        .one(database)
        .await?;

    let number = match last_entry {
        Some(entry) => entry
            .number
            .checked_add(1)
            .ok_or_else(|| DbErr::Custom("entry number overflow".to_owned()))?,
        None => 1,
    };

    let now = unix_timestamp()?;

    entity::entry::ActiveModel {
        id: NotSet,
        event_id: Set(event_id),
        number: Set(number),
        name: Set(name),
        description: Set(description),
        image_filename: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
    }
    .insert(database)
    .await
}

pub async fn entry_by_id(
    database: &DatabaseConnection,
    entry_id: i32,
) -> Result<Option<EntryModel>, DbErr> {
    entity::entry::Entity::find_by_id(entry_id)
        .one(database)
        .await
}

pub async fn update_entry(
    database: &DatabaseConnection,
    entry_id: i32,
    name: String,
    description: Option<String>,
) -> Result<Option<EntryModel>, DbErr> {
    let Some(entry) = entity::entry::Entity::find_by_id(entry_id)
        .one(database)
        .await?
    else {
        return Ok(None);
    };

    let now = unix_timestamp()?;

    let mut active_entry: entity::entry::ActiveModel = entry.into();

    active_entry.name = Set(name);
    active_entry.description = Set(description);
    active_entry.updated_at = Set(now);

    Ok(Some(active_entry.update(database).await?))
}

pub async fn set_entry_image_filename(
    database: &DatabaseConnection,
    entry_id: i32,
    image_filename: Option<String>,
) -> Result<Option<EntryModel>, DbErr> {
    let Some(entry) = entity::entry::Entity::find_by_id(entry_id)
        .one(database)
        .await?
    else {
        return Ok(None);
    };

    let now = unix_timestamp()?;

    let mut active_entry: entity::entry::ActiveModel = entry.into();

    active_entry.image_filename = Set(image_filename);
    active_entry.updated_at = Set(now);

    Ok(Some(active_entry.update(database).await?))
}

pub use entity::vote::Model as VoteModel;

pub async fn current_vote(
    database: &DatabaseConnection,
    event_id: i32,
    voter_hash: &str,
) -> Result<Option<VoteModel>, DbErr> {
    entity::vote::Entity::find()
        .filter(entity::vote::Column::EventId.eq(event_id))
        .filter(entity::vote::Column::VoterHash.eq(voter_hash))
        .one(database)
        .await
}

pub async fn set_vote(
    database: &DatabaseConnection,
    event_id: i32,
    voter_hash: String,
    entry_id: i32,
) -> Result<VoteModel, DbErr> {
    let existing = entity::vote::Entity::find()
        .filter(entity::vote::Column::EventId.eq(event_id))
        .filter(entity::vote::Column::VoterHash.eq(voter_hash.clone()))
        .one(database)
        .await?;

    let now = unix_timestamp()?;

    if let Some(existing) = existing {
        let mut active_vote: entity::vote::ActiveModel = existing.into();

        active_vote.entry_id = Set(entry_id);
        active_vote.updated_at = Set(now);

        return active_vote.update(database).await;
    }

    entity::vote::ActiveModel {
        id: NotSet,
        event_id: Set(event_id),
        voter_hash: Set(voter_hash),
        entry_id: Set(entry_id),
        created_at: Set(now),
        updated_at: Set(now),
    }
    .insert(database)
    .await
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
            schema
                .has_table("events")
                .await
                .expect("events schema check should succeed")
        );

        assert!(
            schema
                .has_table("entries")
                .await
                .expect("entries schema check should succeed")
        );

        assert!(
            schema
                .has_table("votes")
                .await
                .expect("votes schema check should succeed")
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

        assert!(
            !event_slug_exists(&database, "cake-beauty-2026",)
                .await
                .expect("event slug lookup should succeed")
        );

        let event = create_event(
            &database,
            "cake-beauty-2026".to_owned(),
            "Cake Beauty 2026".to_owned(),
            Some("Very serious cake business.".to_owned()),
        )
        .await
        .expect("event should be created");

        assert_eq!(event.slug, "cake-beauty-2026");
        assert_eq!(event.status, "draft");
        assert!(!event.results_public);

        let fetched_by_slug = event_by_slug(&database, "cake-beauty-2026")
            .await
            .expect("event slug lookup should succeed")
            .expect("created event should exist");

        assert_eq!(fetched_by_slug.id, event.id);

        assert!(
            event_slug_exists(&database, "cake-beauty-2026",)
                .await
                .expect("event slug lookup should succeed")
        );

        let events = list_events(&database).await.expect("events should list");

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].title, "Cake Beauty 2026");

        let fetched = event_by_id(&database, event.id)
            .await
            .expect("event lookup should succeed")
            .expect("created event should exist");

        assert_eq!(fetched.slug, "cake-beauty-2026");

        let first_entry = create_entry(
            &database,
            event.id,
            "Strawberry Cake".to_owned(),
            Some("Pink and suspiciously beautiful.".to_owned()),
        )
        .await
        .expect("first entry should be created");

        assert_eq!(first_entry.event_id, event.id);
        assert_eq!(first_entry.number, 1);
        assert!(first_entry.image_filename.is_none());

        let second_entry = create_entry(&database, event.id, "Chocolate Cake".to_owned(), None)
            .await
            .expect("second entry should be created");

        assert_eq!(second_entry.number, 2);

        let entries = list_entries(&database, event.id)
            .await
            .expect("entries should list");

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].number, 1);
        assert_eq!(entries[0].name, "Strawberry Cake");
        assert_eq!(entries[1].number, 2);
        assert_eq!(entries[1].name, "Chocolate Cake");

        let edited_entry = update_entry(
            &database,
            first_entry.id,
            "Strawberry Masterpiece".to_owned(),
            Some("Updated description.".to_owned()),
        )
        .await
        .expect("entry update should succeed")
        .expect("entry should exist");

        assert_eq!(edited_entry.name, "Strawberry Masterpiece");

        let imaged_entry =
            set_entry_image_filename(&database, first_entry.id, Some("test-image.jpg".to_owned()))
                .await
                .expect("entry image update should succeed")
                .expect("entry should exist");

        assert_eq!(
            imaged_entry.image_filename.as_deref(),
            Some("test-image.jpg")
        );

        let voter_hash = "test-browser-voter-hash";

        assert!(
            current_vote(&database, event.id, voter_hash,)
                .await
                .expect("vote lookup should succeed")
                .is_none()
        );

        let first_vote = set_vote(&database, event.id, voter_hash.to_owned(), first_entry.id)
            .await
            .expect("first vote should succeed");

        assert_eq!(first_vote.entry_id, first_entry.id);

        let changed_vote = set_vote(&database, event.id, voter_hash.to_owned(), second_entry.id)
            .await
            .expect("vote change should succeed");

        assert_eq!(changed_vote.id, first_vote.id);

        assert_eq!(changed_vote.entry_id, second_entry.id);

        let current = current_vote(&database, event.id, voter_hash)
            .await
            .expect("vote lookup should succeed")
            .expect("vote should exist");

        assert_eq!(current.entry_id, second_entry.id);

        let updated = update_event(
            &database,
            event.id,
            "Cake Beauty Championship 2026".to_owned(),
            Some("Updated cake business.".to_owned()),
            "open".to_owned(),
            true,
        )
        .await
        .expect("event update should succeed")
        .expect("created event should still exist");

        assert_eq!(updated.title, "Cake Beauty Championship 2026");
        assert_eq!(updated.slug, "cake-beauty-2026");
        assert_eq!(updated.status, "open");
        assert!(updated.results_public);

        assert!(
            event_by_id(&database, 999_999)
                .await
                .expect("missing event lookup should succeed")
                .is_none()
        );
    }
}
