use std::time::Duration;

use mongodb::bson::doc;
use rocket::{serde::json::Json, State};
use serde::Serialize;
use tokio::time::timeout;

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
    let timeout_duration = Duration::new(5, 0); // 5 seconds
    let result = timeout(
        timeout_duration,
        state.mongodb.run_command(doc! {"ping": 1}),
    )
    .await;

    let database_connected = match result {
        Ok(Ok(_)) => true,   // Command succeeded within the timeout
        Ok(Err(_)) => false, // Command failed (e.g., MongoDB not reachable)
        Err(_) => false,     // Command timed out
    };
    Json(DbCheckResponse { database_connected })
}

#[get("/prometheus")]
pub async fn prometheus() -> String {
    //TODO : Add prometheus metrics
    String::from("TODO : Add prometheus metrics")
}
