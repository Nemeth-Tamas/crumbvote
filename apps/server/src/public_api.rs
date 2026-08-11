use crate::{
    AppState,
    admin::{ApiError, api_error},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::Serialize;

#[derive(Serialize)]
struct PublicEventResponse {
    slug: String,
    title: String,
    description: Option<String>,
    status: String,
    results_public: bool,
}

#[derive(Serialize)]
struct PublicEntryDetails {
    id: i32,
    number: i32,
    name: String,
    description: Option<String>,
    image_url: Option<String>,
}

#[derive(Serialize)]
struct PublicEntryResponse {
    event: PublicEventResponse,
    entry: PublicEntryDetails,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/events/{slug}/entries/{entry_id}", get(get_public_entry))
}

async fn get_public_entry(
    State(state): State<AppState>,
    Path((slug, entry_id)): Path<(String, i32)>,
) -> Result<Json<PublicEntryResponse>, ApiError> {
    let event = crumbvote_database::event_by_slug(&state.database, &slug)
        .await
        .map_err(|error| {
            eprintln!("Failed to load public event: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    let Some(event) = event else {
        return Err(public_entry_not_found());
    };

    let entry = crumbvote_database::entry_by_id(&state.database, entry_id)
        .await
        .map_err(|error| {
            eprintln!("Failed to load public entry: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    let Some(entry) = entry else {
        return Err(public_entry_not_found());
    };

    if entry.event_id != event.id {
        return Err(public_entry_not_found());
    }

    let image_url = entry
        .image_filename
        .map(|filename| format!("/media/entries/{filename}"));

    Ok(Json(PublicEntryResponse {
        event: PublicEventResponse {
            slug: event.slug,
            title: event.title,
            description: event.description,
            status: event.status,
            results_public: event.results_public,
        },

        entry: PublicEntryDetails {
            id: entry.id,
            number: entry.number,
            name: entry.name,
            description: entry.description,
            image_url,
        },
    }))
}

fn public_entry_not_found() -> ApiError {
    api_error(StatusCode::NOT_FOUND, "public_entry_not_found")
}
