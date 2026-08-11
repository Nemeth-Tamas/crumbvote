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
use serde::Serialize;
use std::collections::{HashMap, HashSet};

#[derive(Serialize)]
struct EntryAnalyticsResponse {
    entry_id: i32,
    scans: usize,
    unique_visitors: usize,
    current_votes: usize,
}

#[derive(Serialize)]
struct EventAnalyticsResponse {
    total_scans: usize,
    unique_visitors: usize,
    current_votes: usize,
    vote_changes: usize,
    entries: Vec<EntryAnalyticsResponse>,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/events/{event_id}/analytics", get(event_analytics))
}

async fn event_analytics(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(event_id): Path<i32>,
) -> Result<Json<EventAnalyticsResponse>, ApiError> {
    require_authenticated(&state, &jar).await?;

    let event = crumbvote_database::event_by_id(&state.database, event_id)
        .await
        .map_err(|error| {
            eprintln!("Failed to load analytics event: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    if event.is_none() {
        return Err(api_error(StatusCode::NOT_FOUND, "event_not_found"));
    }

    let entries = crumbvote_database::list_entries(&state.database, event_id)
        .await
        .map_err(|error| {
            eprintln!("Failed to load analytics entries: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    let votes = crumbvote_database::list_votes(&state.database, event_id)
        .await
        .map_err(|error| {
            eprintln!("Failed to load analytics votes: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    let activities = crumbvote_database::list_activity_events(&state.database, event_id)
        .await
        .map_err(|error| {
            eprintln!("Failed to load analytics activity: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    let mut total_scans = 0;
    let mut vote_changes = 0;

    let mut unique_visitors = HashSet::new();

    let mut entry_scans: HashMap<i32, usize> = HashMap::new();

    let mut entry_visitors: HashMap<i32, HashSet<String>> = HashMap::new();

    for activity in &activities {
        match activity.kind.as_str() {
            "scan" => {
                total_scans += 1;

                unique_visitors.insert(activity.voter_hash.clone());

                *entry_scans.entry(activity.entry_id).or_default() += 1;

                entry_visitors
                    .entry(activity.entry_id)
                    .or_default()
                    .insert(activity.voter_hash.clone());
            }

            "vote_change" => {
                vote_changes += 1;
            }

            _ => {}
        }
    }

    let mut entry_votes: HashMap<i32, usize> = HashMap::new();

    for vote in &votes {
        *entry_votes.entry(vote.entry_id).or_default() += 1;
    }

    let entry_analytics = entries
        .into_iter()
        .map(|entry| EntryAnalyticsResponse {
            entry_id: entry.id,

            scans: entry_scans.get(&entry.id).copied().unwrap_or(0),

            unique_visitors: entry_visitors.get(&entry.id).map(HashSet::len).unwrap_or(0),

            current_votes: entry_votes.get(&entry.id).copied().unwrap_or(0),
        })
        .collect();

    Ok(Json(EventAnalyticsResponse {
        total_scans,
        unique_visitors: unique_visitors.len(),
        current_votes: votes.len(),
        vote_changes,
        entries: entry_analytics,
    }))
}
