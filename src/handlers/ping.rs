use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
}

#[derive(Serialize)]
pub struct DbCheckResponse {
    database_connected: bool,
}

#[get("/ping")]
pub fn ping() -> &'static str {
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
    format!("# HELP request_count Total number of requests\n# TYPE request_count counter\nrequest_count")
}

#[get("/db-check")]
pub fn db_check() -> Json<DbCheckResponse> {
    // Simulated database connection check
    let database_connected = true; // Replace with actual connection logic
    Json(DbCheckResponse { database_connected })
}

#[get("/prometheus")]
pub fn prometheus() -> String {
    format!("# HELP request_count Total number of requests\n# TYPE request_count counter\nrequest_count")
}
