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

const RECENT_ACTIVITY_LIMIT: usize = 12;
const HIGH_SCAN_THRESHOLD: usize = 10;
const HIGH_VOTE_CHANGE_THRESHOLD: usize = 3;

#[derive(Serialize)]
struct EntryAnalyticsResponse {
    entry_id: i32,
    scans: usize,
    unique_visitors: usize,
    current_votes: usize,
}

#[derive(Serialize)]
struct RecentActivityResponse {
    kind: String,
    entry_id: i32,
    created_at: i64,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
struct AnalyticsSignalResponse {
    code: &'static str,
    affected_visitors: usize,
}

#[derive(Serialize)]
struct EventAnalyticsResponse {
    total_scans: usize,
    unique_visitors: usize,
    current_votes: usize,
    vote_changes: usize,
    entries: Vec<EntryAnalyticsResponse>,
    recent_activity: Vec<RecentActivityResponse>,
    signals: Vec<AnalyticsSignalResponse>,
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

    let mut visitor_scans: HashMap<String, usize> = HashMap::new();

    let mut visitor_vote_changes: HashMap<String, usize> = HashMap::new();

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

                *visitor_scans
                    .entry(activity.voter_hash.clone())
                    .or_default() += 1;
            }

            "vote_change" => {
                vote_changes += 1;

                *visitor_vote_changes
                    .entry(activity.voter_hash.clone())
                    .or_default() += 1;
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

    let recent_activity = activities
        .iter()
        .rev()
        .take(RECENT_ACTIVITY_LIMIT)
        .map(|activity| RecentActivityResponse {
            kind: activity.kind.clone(),
            entry_id: activity.entry_id,
            created_at: activity.created_at,
        })
        .collect();

    let signals = build_signals(&visitor_scans, &visitor_vote_changes);

    Ok(Json(EventAnalyticsResponse {
        total_scans,
        unique_visitors: unique_visitors.len(),
        current_votes: votes.len(),
        vote_changes,
        entries: entry_analytics,
        recent_activity,
        signals,
    }))
}

fn build_signals(
    visitor_scans: &HashMap<String, usize>,
    visitor_vote_changes: &HashMap<String, usize>,
) -> Vec<AnalyticsSignalResponse> {
    let high_scan_repeaters = visitor_scans
        .values()
        .filter(|count| **count >= HIGH_SCAN_THRESHOLD)
        .count();

    let frequent_vote_changers = visitor_vote_changes
        .values()
        .filter(|count| **count >= HIGH_VOTE_CHANGE_THRESHOLD)
        .count();

    let mut signals = Vec::new();

    if high_scan_repeaters > 0 {
        signals.push(AnalyticsSignalResponse {
            code: "high_scan_repeaters",
            affected_visitors: high_scan_repeaters,
        });
    }

    if frequent_vote_changers > 0 {
        signals.push(AnalyticsSignalResponse {
            code: "frequent_vote_changers",
            affected_visitors: frequent_vote_changers,
        });
    }

    signals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analytics_signals_use_review_thresholds() {
        let mut scans = HashMap::new();
        scans.insert("busy-scanner".to_owned(), HIGH_SCAN_THRESHOLD);

        let mut vote_changes = HashMap::new();
        vote_changes.insert("busy-changer".to_owned(), HIGH_VOTE_CHANGE_THRESHOLD);

        let signals = build_signals(&scans, &vote_changes);

        assert_eq!(
            signals,
            vec![
                AnalyticsSignalResponse {
                    code: "high_scan_repeaters",
                    affected_visitors: 1,
                },
                AnalyticsSignalResponse {
                    code: "frequent_vote_changers",
                    affected_visitors: 1,
                },
            ]
        );
    }
}
