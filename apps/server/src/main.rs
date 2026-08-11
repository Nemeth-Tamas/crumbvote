mod admin;
mod auth;
mod entries;
mod events;

use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use crumbvote_database::DatabaseConnection;
use serde::Serialize;
use std::{
    error::Error,
    sync::{Arc, Mutex},
};
use tower_http::services::ServeDir;

const DEFAULT_LISTEN_ADDRESS: &str = "0.0.0.0:3000";

const DEFAULT_DATABASE_URL: &str = "sqlite://data/crumbvote.sqlite?mode=rwc";

const DATA_DIRECTORY: &str = "data";

pub(crate) const ENTRY_IMAGE_DIRECTORY: &str = "data/uploads/entries";

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) database: DatabaseConnection,
    pub(crate) setup_code: Arc<Mutex<Option<String>>>,
    pub(crate) secure_cookies: bool,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
    database: &'static str,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::fs::create_dir_all(DATA_DIRECTORY)?;
    std::fs::create_dir_all(ENTRY_IMAGE_DIRECTORY)?;

    let listen_address = std::env::var("CRUMBVOTE_LISTEN_ADDRESS")
        .unwrap_or_else(|_| DEFAULT_LISTEN_ADDRESS.to_owned());

    let database_url =
        std::env::var("CRUMBVOTE_DATABASE_URL").unwrap_or_else(|_| DEFAULT_DATABASE_URL.to_owned());

    let database = crumbvote_database::connect(&database_url).await?;

    let setup_code = if crumbvote_database::admin_is_configured(&database).await? {
        None
    } else {
        let code = if std::env::var_os("PLAYWRIGHT_TEST").is_some() {
            std::env::var("CRUMBVOTE_E2E_SETUP_CODE").map_err(|_| {
                std::io::Error::other(
                    "PLAYWRIGHT_TEST requires \
                         CRUMBVOTE_E2E_SETUP_CODE",
                )
            })?
        } else {
            auth::generate_setup_code().map_err(|error| {
                std::io::Error::other(format!(
                    "failed to generate admin \
                             setup code: {error}"
                ))
            })?
        };

        println!();
        println!("==========================================");
        println!(" CrumbVote first-time setup required");
        println!();
        println!(" Admin setup code: {code}");
        println!();
        println!(" Open /admin to configure CrumbVote.");
        println!("==========================================");
        println!();

        Some(code)
    };

    let secure_cookies = std::env::var("CRUMBVOTE_SECURE_COOKIES")
        .is_ok_and(|value| value == "1" || value.eq_ignore_ascii_case("true"));

    let state = AppState {
        database,
        setup_code: Arc::new(Mutex::new(setup_code)),
        secure_cookies,
    };

    let admin_api = admin::router()
        .merge(events::router())
        .merge(entries::router());

    let app = Router::new()
        .route("/health", get(health))
        .nest("/api/admin", admin_api)
        .nest_service("/media/entries", ServeDir::new(ENTRY_IMAGE_DIRECTORY))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&listen_address).await?;

    println!("CrumbVote listening on http://{listen_address}");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health(State(state): State<AppState>) -> Result<Json<HealthResponse>, StatusCode> {
    state
        .database
        .ping()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    Ok(Json(HealthResponse {
        status: "ok",
        service: "crumbvote",
        database: "ok",
    }))
}
