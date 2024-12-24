use mongodb::bson::doc;
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
    let database_connected = state.mongodb.run_command(doc! {"ping": 1}).await.is_ok();

    Json(DbCheckResponse { database_connected })
}>

#[get("/prometheus")]
pub async fn prometheus() -> String {
    //TODO : Add prometheus metrics
    String::from("TODO : Add prometheus metrics")
}
