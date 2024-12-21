use log::info;
use rocket::{serde::json::Json, State};
use serde::Serialize;

use crate::{config::AppConfig, guard::basic_auth::BasicAuth};

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
}

#[derive(Serialize)]
pub struct DbCheckResponse {
    database_connected: bool,
}

#[get("/ping")]
pub fn ping(base: BasicAuth) -> &'static str {
    info!("User: {}", base.username);
    "pong"
}

#[get("/health")]
pub fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
    })
}

#[get("/metrics")]
pub fn metrics() -> String {
    String::from("TODO : Add metrics")
}

#[get("/db-check")]
pub fn db_check(state: &State<AppConfig>) -> Json<DbCheckResponse> {
    // TODO Check all databases connection
    let database_connected = true; // Replace with actual connection logic
    Json(DbCheckResponse { database_connected })
}

#[get("/prometheus")]
pub fn prometheus() -> String {
    String::from("TODO : Add prometheus metrics")
}
