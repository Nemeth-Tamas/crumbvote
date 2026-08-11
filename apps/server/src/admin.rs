use crate::{AppState, auth};
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use serde::{Deserialize, Serialize};

const MIN_PASSWORD_CHARACTERS: usize = 12;
const MAX_PASSWORD_BYTES: usize = 256;

const ADMIN_SESSION_COOKIE: &str = "crumbvote_admin_session";

const ADMIN_SESSION_TTL_SECONDS: i64 = 60 * 60 * 24 * 7;

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

#[derive(Deserialize)]
struct LoginRequest {
    password: String,
}

#[derive(Serialize)]
struct SessionResponse {
    authenticated: bool,
}

#[derive(Serialize)]
pub(crate) struct ErrorResponse {
    error: &'static str,
}

pub(crate) type ApiError = (StatusCode, Json<ErrorResponse>);

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/status", get(status))
        .route("/setup", post(setup))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/session", get(session))
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

async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> Result<(CookieJar, Json<SessionResponse>), ApiError> {
    if request.password.len() > MAX_PASSWORD_BYTES {
        return Err(api_error(StatusCode::BAD_REQUEST, "password_too_long"));
    }

    let password_hash = crumbvote_database::admin_password_hash(&state.database)
        .await
        .map_err(|error| {
            eprintln!("Failed to load admin credential: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    let Some(password_hash) = password_hash else {
        return Err(api_error(StatusCode::CONFLICT, "setup_required"));
    };

    let password = request.password;

    let password_matches = tokio::task::spawn_blocking(move || {
        auth::verify_password(&password, &password_hash).map_err(|error| error.to_string())
    })
    .await
    .map_err(|error| {
        eprintln!("Password verification task failed: {error}");

        api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "password_verification_failed",
        )
    })?
    .map_err(|error| {
        eprintln!("Password verification failed: {error}");

        api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "password_verification_failed",
        )
    })?;

    if !password_matches {
        return Err(api_error(StatusCode::UNAUTHORIZED, "invalid_credentials"));
    }

    let session_token = auth::generate_session_token().map_err(|error| {
        eprintln!("Failed to generate admin session token: {error}");

        api_error(StatusCode::INTERNAL_SERVER_ERROR, "session_creation_failed")
    })?;

    let token_hash = auth::hash_session_token(&session_token);

    crumbvote_database::create_admin_session(
        &state.database,
        token_hash,
        ADMIN_SESSION_TTL_SECONDS,
    )
    .await
    .map_err(|error| {
        eprintln!("Failed to persist admin session: {error}");

        api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
    })?;

    let jar = add_session_cookie(jar, session_token, state.secure_cookies);

    Ok((
        jar,
        Json(SessionResponse {
            authenticated: true,
        }),
    ))
}

async fn session(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, Json<SessionResponse>), ApiError> {
    let Some(session_token) = jar
        .get(ADMIN_SESSION_COOKIE)
        .map(|cookie| cookie.value().to_owned())
    else {
        return Ok((
            jar,
            Json(SessionResponse {
                authenticated: false,
            }),
        ));
    };

    let token_hash = auth::hash_session_token(&session_token);

    let valid = crumbvote_database::admin_session_is_valid(&state.database, &token_hash)
        .await
        .map_err(|error| {
            eprintln!("Failed to validate admin session: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    if !valid {
        return Ok((
            remove_session_cookie(jar),
            Json(SessionResponse {
                authenticated: false,
            }),
        ));
    }

    Ok((
        jar,
        Json(SessionResponse {
            authenticated: true,
        }),
    ))
}

async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, Json<SessionResponse>), ApiError> {
    if let Some(session_token) = jar
        .get(ADMIN_SESSION_COOKIE)
        .map(|cookie| cookie.value().to_owned())
    {
        let token_hash = auth::hash_session_token(&session_token);

        crumbvote_database::delete_admin_session(&state.database, &token_hash)
            .await
            .map_err(|error| {
                eprintln!("Failed to delete admin session: {error}");

                api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
            })?;
    }

    Ok((
        remove_session_cookie(jar),
        Json(SessionResponse {
            authenticated: false,
        }),
    ))
}

fn add_session_cookie(jar: CookieJar, session_token: String, secure: bool) -> CookieJar {
    let cookie = Cookie::build((ADMIN_SESSION_COOKIE, session_token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        .secure(secure)
        .max_age(time::Duration::seconds(ADMIN_SESSION_TTL_SECONDS))
        .build();

    jar.add(cookie)
}

fn remove_session_cookie(jar: CookieJar) -> CookieJar {
    jar.remove(Cookie::build(ADMIN_SESSION_COOKIE).path("/").build())
}

pub(crate) async fn require_authenticated(
    state: &AppState,
    jar: &CookieJar,
) -> Result<(), ApiError> {
    let Some(session_token) = jar
        .get(ADMIN_SESSION_COOKIE)
        .map(|cookie| cookie.value().to_owned())
    else {
        return Err(api_error(
            StatusCode::UNAUTHORIZED,
            "authentication_required",
        ));
    };

    let token_hash = auth::hash_session_token(&session_token);

    let valid = crumbvote_database::admin_session_is_valid(&state.database, &token_hash)
        .await
        .map_err(|error| {
            eprintln!("Failed to validate admin session: {error}");

            api_error(StatusCode::INTERNAL_SERVER_ERROR, "database_error")
        })?;

    if !valid {
        return Err(api_error(
            StatusCode::UNAUTHORIZED,
            "authentication_required",
        ));
    }

    Ok(())
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

pub(crate) fn api_error(status: StatusCode, error: &'static str) -> ApiError {
    (status, Json(ErrorResponse { error }))
}
