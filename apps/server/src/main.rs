use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use crumbvote_database::DatabaseConnection;
use serde::Serialize;
use std::error::Error;

const LISTEN_ADDRESS: &str = "0.0.0.0:3000";

const DEFAULT_DATABASE_URL: &str = "sqlite://data/crumbvote.sqlite?mode=rwc";

const DATA_DIRECTORY: &str = "data";

#[derive(Clone)]
struct AppState {
    database: DatabaseConnection,
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

    let database_url =
        std::env::var("CRUMBVOTE_DATABASE_URL").unwrap_or_else(|_| DEFAULT_DATABASE_URL.to_owned());

    let database = crumbvote_database::connect(&database_url).await?;

    let state = AppState { database };

    let app = Router::new()
        .route("/health", get(health))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(LISTEN_ADDRESS).await?;

    println!("CrumbVote listening on http://{LISTEN_ADDRESS}");

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
