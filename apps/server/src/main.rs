use axum::{Json, Router, routing::get};
use serde::Serialize;

const LISTEN_ADDRESS: &str = "0.0.0.0:3000";

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new().route("/health", get(health));

    let listener = tokio::net::TcpListener::bind(LISTEN_ADDRESS).await?;

    println!("CrumbVote listening on http://{LISTEN_ADDRESS}");

    axum::serve(listener, app).await
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "crumbvote",
    })
}
