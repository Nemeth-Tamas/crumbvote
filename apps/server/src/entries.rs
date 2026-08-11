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

const MAX_NAME_CHARACTERS: usize = 120;
const MAX_DESCRIPTION_CHARACTERS: usize = 2000;

#[derive(Deserialize)]
struct CreateEntryRequest {
    name: String,
    description: Option<String>,
}

#[derive(Serialize)]
struct EntryResponse {
    id: i32,
    event_id: i32,
    number: i32,
    name: String,
    description: Option<String>,
    created_at: i64,
    updated_at: i64,
}

impl From<crumbvote_database::EntryModel> for EntryResponse {
    fn from(entry: crumbvote_database::EntryModel) -> Self {
        Self {
            id: entry.id,
            event_id: entry.event_id,
            number: entry.number,
            name: entry.name,
            description: entry.description,
            created_at: entry.created_at,
            updated_at: entry.updated_at,
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/events/{event_id}/entries",
        get(list_entries).post(create_entry),
    )
}

async fn list_entries(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(event_id): Path<i32>,
) -> Result<Json<Vec<EntryResponse>>, ApiError> {
    require_authenticated(&state, &jar).await?;

    require_event(&state, event_id).await?;

    let entries = crumbvote_database::list_entries(&state.database, event_id)
        .await
        .map_err(|error| {
            eprintln!("Failed to list entries: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    Ok(Json(entries.into_iter().map(EntryResponse::from).collect()))
}

async fn create_entry(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(event_id): Path<i32>,
    Json(request): Json<CreateEntryRequest>,
) -> Result<(StatusCode, Json<EntryResponse>), ApiError> {
    require_authenticated(&state, &jar).await?;

    require_event(&state, event_id).await?;

    let name = request.name.trim().to_owned();

    validate_name(&name)?;

    let description = request
        .description
        .map(|description| description.trim().to_owned())
        .filter(|description| !description.is_empty());

    validate_description(&description)?;

    let entry = crumbvote_database::create_entry(&state.database, event_id, name, description)
        .await
        .map_err(|error| {
            eprintln!("Failed to create entry: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    Ok((StatusCode::CREATED, Json(EntryResponse::from(entry))))
}

async fn require_event(state: &AppState, event_id: i32) -> Result<(), ApiError> {
    let event = crumbvote_database::event_by_id(&state.database, event_id)
        .await
        .map_err(|error| {
            eprintln!("Failed to load event: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    if event.is_none() {
        return Err(api_error(StatusCode::NOT_FOUND, "event_not_found"));
    }

    Ok(())
}

fn validate_name(name: &str) -> Result<(), ApiError> {
    if name.is_empty() {
        return Err(api_error(StatusCode::BAD_REQUEST, "entry_name_required"));
    }

    if name.chars().count() > MAX_NAME_CHARACTERS {
        return Err(api_error(StatusCode::BAD_REQUEST, "entry_name_too_long"));
    }

    Ok(())
}

fn validate_description(description: &Option<String>) -> Result<(), ApiError> {
    if description
        .as_ref()
        .is_some_and(|description| description.chars().count() > MAX_DESCRIPTION_CHARACTERS)
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "entry_description_too_long",
        ));
    }

    Ok(())
}
