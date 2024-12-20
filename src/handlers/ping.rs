use rocket::{serde::json::Json, State};
use serde::Serialize;

use crate::config::AppConfig;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
}

#[derive(Serialize)]
pub struct DbCheckResponse {
    database_connected: bool,
}

#[get("/ping")]
pub async fn ping() -> &'static str {
    "pong"
}

#[get("/health")]
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
    })
}

#[get("/metrics")]
pub async fn metrics() -> String {
    String::from("TODO : Add metrics")
}

#[get("/db-check")]
pub async fn db_check(state: &State<AppConfig>) -> Json<DbCheckResponse> {
    // TODO Check all databases connection
    let database_connected = true; // Replace with actual connection logic
    Json(DbCheckResponse { database_connected })
}

#[get("/prometheus")]
pub async fn prometheus() -> String {
    String::from("TODO : Add prometheus metrics")
}
