use crate::{
    AppState,
    admin::{ApiError, api_error},
    auth,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use serde::{Deserialize, Serialize};

const VOTER_COOKIE: &str = "crumbvote_voter";

const VOTER_HEADER: &str = "x-crumbvote-voter";

const VOTER_TOKEN_LENGTH: usize = 64;

const VOTER_TTL_SECONDS: i64 = 60 * 60 * 24 * 365;

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

#[derive(Deserialize)]
struct EnsureVoterRequest {
    token: Option<String>,
}

#[derive(Serialize)]
struct VoterIdentityResponse {
    token: String,
}

#[derive(Deserialize)]
struct CastVoteRequest {
    entry_id: i32,
}

#[derive(Serialize)]
struct VoteResponse {
    entry_id: Option<i32>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/voter", post(ensure_voter))
        .route("/events/{slug}/entries/{entry_id}", get(get_public_entry))
        .route("/events/{slug}/vote", get(get_current_vote).post(cast_vote))
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

async fn ensure_voter(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<EnsureVoterRequest>,
) -> Result<(CookieJar, Json<VoterIdentityResponse>), ApiError> {
    let token = if let Some(requested) = request.token {
        let requested = requested.trim().to_owned();

        validate_voter_token(&requested)?;

        requested
    } else if let Some(cookie_token) = jar
        .get(VOTER_COOKIE)
        .map(|cookie| cookie.value().to_owned())
    {
        if voter_token_is_valid(&cookie_token) {
            cookie_token
        } else {
            generate_voter_token()?
        }
    } else {
        generate_voter_token()?
    };

    let jar = add_voter_cookie(jar, token.clone(), state.secure_cookies);

    Ok((jar, Json(VoterIdentityResponse { token })))
}

async fn get_current_vote(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    headers: HeaderMap,
) -> Result<Json<VoteResponse>, ApiError> {
    let event = require_public_event(&state, &slug).await?;

    let voter_token = voter_token_from_headers(&headers)?;

    let voter_hash = auth::hash_token(&voter_token);

    let vote = crumbvote_database::current_vote(&state.database, event.id, &voter_hash)
        .await
        .map_err(|error| {
            eprintln!("Failed to load current vote: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    Ok(Json(VoteResponse {
        entry_id: vote.map(|vote| vote.entry_id),
    }))
}

async fn cast_vote(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    headers: HeaderMap,
    Json(request): Json<CastVoteRequest>,
) -> Result<Json<VoteResponse>, ApiError> {
    let event = require_public_event(&state, &slug).await?;

    if event.status != "open" {
        return Err(api_error(StatusCode::CONFLICT, "voting_not_open"));
    }

    let entry = require_public_entry(&state, event.id, request.entry_id).await?;

    let voter_token = voter_token_from_headers(&headers)?;

    let voter_hash = auth::hash_token(&voter_token);

    let vote = crumbvote_database::set_vote(&state.database, event.id, voter_hash, entry.id)
        .await
        .map_err(|error| {
            eprintln!("Failed to persist vote: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    Ok(Json(VoteResponse {
        entry_id: Some(vote.entry_id),
    }))
}

async fn require_public_event(
    state: &AppState,
    slug: &str,
) -> Result<crumbvote_database::EventModel, ApiError> {
    let event = crumbvote_database::event_by_slug(&state.database, slug)
        .await
        .map_err(|error| {
            eprintln!("Failed to load public event: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    event.ok_or_else(public_entry_not_found)
}

async fn require_public_entry(
    state: &AppState,
    event_id: i32,
    entry_id: i32,
) -> Result<crumbvote_database::EntryModel, ApiError> {
    let entry = crumbvote_database::entry_by_id(&state.database, entry_id)
        .await
        .map_err(|error| {
            eprintln!("Failed to load public entry: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    match entry {
        Some(entry) if entry.event_id == event_id => Ok(entry),

        _ => Err(public_entry_not_found()),
    }
}

fn voter_token_from_headers(headers: &HeaderMap) -> Result<String, ApiError> {
    let value = headers
        .get(VOTER_HEADER)
        .ok_or_else(|| api_error(StatusCode::BAD_REQUEST, "voter_token_required"))?
        .to_str()
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "invalid_voter_token"))?
        .to_owned();

    validate_voter_token(&value)?;

    Ok(value)
}

fn validate_voter_token(token: &str) -> Result<(), ApiError> {
    if !voter_token_is_valid(token) {
        return Err(api_error(StatusCode::BAD_REQUEST, "invalid_voter_token"));
    }

    Ok(())
}

fn voter_token_is_valid(token: &str) -> bool {
    token.len() == VOTER_TOKEN_LENGTH
        && token.chars().all(|character| character.is_ascii_hexdigit())
}

fn generate_voter_token() -> Result<String, ApiError> {
    auth::generate_random_token().map_err(|error| {
        eprintln!("Failed to generate voter token: {error}");

        api_error(StatusCode::INTERNAL_SERVER_ERROR, "voter_creation_failed")
    })
}

fn add_voter_cookie(jar: CookieJar, token: String, secure: bool) -> CookieJar {
    let cookie = Cookie::build((VOTER_COOKIE, token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        .secure(secure)
        .max_age(time::Duration::seconds(VOTER_TTL_SECONDS))
        .build();

    jar.add(cookie)
}

fn public_entry_not_found() -> ApiError {
    api_error(StatusCode::NOT_FOUND, "public_entry_not_found")
}
