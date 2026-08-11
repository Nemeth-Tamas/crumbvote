use crate::{AppState, auth};
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

const MIN_PASSWORD_CHARACTERS: usize = 12;
const MAX_PASSWORD_BYTES: usize = 256;

#[derive(Serialize)]
struct AdminStatusResponse {
    setup_required: bool,
}

#[derive(Deserialize)]
struct SetupRequest {
    setup_code: String,
    password: String,
}

#[derive(Serialize)]
struct SetupResponse {
    configured: bool,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: &'static str,
}

type ApiError = (StatusCode, Json<ErrorResponse>);

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/status", get(status))
        .route("/setup", post(setup))
}

async fn status(State(state): State<AppState>) -> Result<Json<AdminStatusResponse>, ApiError> {
    let configured = crumbvote_database::admin_is_configured(&state.database)
        .await
        .map_err(|error| {
            eprintln!("Failed to check admin configuration: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    Ok(Json(AdminStatusResponse {
        setup_required: !configured,
    }))
}

async fn setup(
    State(state): State<AppState>,
    Json(request): Json<SetupRequest>,
) -> Result<Json<SetupResponse>, ApiError> {
    let already_configured = crumbvote_database::admin_is_configured(&state.database)
        .await
        .map_err(|error| {
            eprintln!("Failed to check admin configuration: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    if already_configured {
        return Err(api_error(StatusCode::CONFLICT, "already_configured"));
    }

    validate_password(&request.password)?;

    let code_matches = {
        let setup_code = state
            .setup_code
            .lock()
            .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "setup_state_unavailable"))?;

        setup_code
            .as_deref()
            .is_some_and(|expected| expected.eq_ignore_ascii_case(request.setup_code.trim()))
    };

    if !code_matches {
        return Err(api_error(StatusCode::FORBIDDEN, "invalid_setup_code"));
    }

    let password = request.password;

    let password_hash = tokio::task::spawn_blocking(move || {
        auth::hash_password(&password).map_err(|error| error.to_string())
    })
    .await
    .map_err(|error| {
        eprintln!("Password hashing task failed: {error}");

        api_error(StatusCode::INTERNAL_SERVER_ERROR, "password_hashing_failed")
    })?
    .map_err(|error| {
        eprintln!("Password hashing failed: {error}");

        api_error(StatusCode::INTERNAL_SERVER_ERROR, "password_hashing_failed")
    })?;

    if let Err(error) =
        crumbvote_database::create_admin_credential(&state.database, password_hash).await
    {
        let configured_after_failure = crumbvote_database::admin_is_configured(&state.database)
            .await
            .unwrap_or(false);

        if configured_after_failure {
            return Err(api_error(StatusCode::CONFLICT, "already_configured"));
        }

        eprintln!("Failed to create admin credential: {error}");

        return Err(api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "database_error",
        ));
    }

    {
        let mut setup_code = state
            .setup_code
            .lock()
            .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "setup_state_unavailable"))?;

        *setup_code = None;
    }

    println!("CrumbVote administrator setup completed.");

    Ok(Json(SetupResponse { configured: true }))
}

fn validate_password(password: &str) -> Result<(), ApiError> {
    if password.chars().count() < MIN_PASSWORD_CHARACTERS {
        return Err(api_error(StatusCode::BAD_REQUEST, "password_too_short"));
    }

    if password.len() > MAX_PASSWORD_BYTES {
        return Err(api_error(StatusCode::BAD_REQUEST, "password_too_long"));
    }

    Ok(())
}

fn api_error(status: StatusCode, error: &'static str) -> ApiError {
    (status, Json(ErrorResponse { error }))
}
