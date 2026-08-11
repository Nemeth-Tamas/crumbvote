use crate::{
    AppState,
    admin::{ApiError, api_error, require_authenticated},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};

const MAX_TITLE_CHARACTERS: usize = 120;
const MAX_SLUG_CHARACTERS: usize = 80;
const MAX_DESCRIPTION_CHARACTERS: usize = 2000;

#[derive(Deserialize)]
struct CreateEventRequest {
    title: String,
    slug: String,
    description: Option<String>,
}

#[derive(Deserialize)]
struct UpdateEventRequest {
    title: String,
    description: Option<String>,
    status: String,
    results_public: bool,
}

#[derive(Serialize)]
struct EventResponse {
    id: i32,
    slug: String,
    title: String,
    description: Option<String>,
    status: String,
    results_public: bool,
    created_at: i64,
    updated_at: i64,
}

impl From<crumbvote_database::EventModel> for EventResponse {
    fn from(event: crumbvote_database::EventModel) -> Self {
        Self {
            id: event.id,
            slug: event.slug,
            title: event.title,
            description: event.description,
            status: event.status,
            results_public: event.results_public,
            created_at: event.created_at,
            updated_at: event.updated_at,
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/events", get(list_events).post(create_event))
        .route("/events/{id}", get(get_event).patch(update_event))
}

async fn list_events(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Json<Vec<EventResponse>>, ApiError> {
    require_authenticated(&state, &jar).await?;

    let events = crumbvote_database::list_events(&state.database)
        .await
        .map_err(|error| {
            eprintln!("Failed to list events: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    Ok(Json(events.into_iter().map(EventResponse::from).collect()))
}

async fn get_event(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(event_id): Path<i32>,
) -> Result<Json<EventResponse>, ApiError> {
    require_authenticated(&state, &jar).await?;

    let event = crumbvote_database::event_by_id(&state.database, event_id)
        .await
        .map_err(|error| {
            eprintln!("Failed to load event: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    let Some(event) = event else {
        return Err(api_error(StatusCode::NOT_FOUND, "event_not_found"));
    };

    Ok(Json(EventResponse::from(event)))
}

async fn create_event(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<CreateEventRequest>,
) -> Result<(StatusCode, Json<EventResponse>), ApiError> {
    require_authenticated(&state, &jar).await?;

    let title = request.title.trim().to_owned();
    let slug = request.slug.trim().to_owned();

    validate_title(&title)?;
    validate_slug(&slug)?;

    let description = request
        .description
        .map(|description| description.trim().to_owned())
        .filter(|description| !description.is_empty());

    validate_description(&description)?;

    let slug_exists = crumbvote_database::event_slug_exists(&state.database, &slug)
        .await
        .map_err(|error| {
            eprintln!("Failed to check event slug: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    if slug_exists {
        return Err(api_error(StatusCode::CONFLICT, "event_slug_taken"));
    }

    let event = crumbvote_database::create_event(&state.database, slug, title, description)
        .await
        .map_err(|error| {
            eprintln!("Failed to create event: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    Ok((StatusCode::CREATED, Json(EventResponse::from(event))))
}

async fn update_event(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(event_id): Path<i32>,
    Json(request): Json<UpdateEventRequest>,
) -> Result<Json<EventResponse>, ApiError> {
    require_authenticated(&state, &jar).await?;

    let title = request.title.trim().to_owned();

    validate_title(&title)?;

    let description = request
        .description
        .map(|description| description.trim().to_owned())
        .filter(|description| !description.is_empty());

    validate_description(&description)?;

    let current = crumbvote_database::event_by_id(&state.database, event_id)
        .await
        .map_err(|error| {
            eprintln!("Failed to load event before update: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    let Some(current) = current else {
        return Err(api_error(StatusCode::NOT_FOUND, "event_not_found"));
    };

    validate_status_transition(&current.status, &request.status)?;

    let updated = crumbvote_database::update_event(
        &state.database,
        event_id,
        title,
        description,
        request.status,
        request.results_public,
    )
    .await
    .map_err(|error| {
        eprintln!("Failed to update event: {error}");

        api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
    })?;

    let Some(updated) = updated else {
        return Err(api_error(StatusCode::NOT_FOUND, "event_not_found"));
    };

    Ok(Json(EventResponse::from(updated)))
}

fn validate_description(description: &Option<String>) -> Result<(), ApiError> {
    if description
        .as_ref()
        .is_some_and(|description| description.chars().count() > MAX_DESCRIPTION_CHARACTERS)
    {
        return Err(api_error(StatusCode::BAD_REQUEST, "description_too_long"));
    }

    Ok(())
}

fn validate_status_transition(current: &str, requested: &str) -> Result<(), ApiError> {
    if !matches!(requested, "draft" | "open" | "closed") {
        return Err(api_error(StatusCode::BAD_REQUEST, "invalid_event_status"));
    }

    let allowed = matches!(
        (current, requested),
        ("draft", "draft")
            | ("draft", "open")
            | ("open", "open")
            | ("open", "closed")
            | ("closed", "closed")
            | ("closed", "open")
    );

    if !allowed {
        return Err(api_error(StatusCode::CONFLICT, "invalid_status_transition"));
    }

    Ok(())
}

fn validate_title(title: &str) -> Result<(), ApiError> {
    if title.is_empty() {
        return Err(api_error(StatusCode::BAD_REQUEST, "title_required"));
    }

    if title.chars().count() > MAX_TITLE_CHARACTERS {
        return Err(api_error(StatusCode::BAD_REQUEST, "title_too_long"));
    }

    Ok(())
}

fn validate_slug(slug: &str) -> Result<(), ApiError> {
    let character_count = slug.chars().count();

    if character_count < 3 {
        return Err(api_error(StatusCode::BAD_REQUEST, "slug_too_short"));
    }

    if character_count > MAX_SLUG_CHARACTERS {
        return Err(api_error(StatusCode::BAD_REQUEST, "slug_too_long"));
    }

    if slug.starts_with('-')
        || slug.ends_with('-')
        || slug.contains("--")
        || !slug.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
    {
        return Err(api_error(StatusCode::BAD_REQUEST, "invalid_slug"));
    }

    Ok(())
}
