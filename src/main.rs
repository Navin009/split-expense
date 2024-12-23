#[macro_use]
extern crate rocket;

use crate::config::AppConfig;
use middleware::logging::LoggerFairing;
use rocket::{Build, Rocket};
use security::jwt::JWTAuthenticator;

mod config;
mod exception;
mod guard;
mod handlers;
mod middleware;
mod models;
mod security;
mod utils;

#[launch]
async fn rocket() -> Rocket<Build> {
    AppConfig::init_logger();

    let app_config = AppConfig::new()
        .await
        .expect("Failed to initialize app config");

    rocket::build()
        .manage(JWTAuthenticator::new())
        .manage(app_config)
        .attach(LoggerFairing)
        .mount("/", handlers::ping_routes())
        .mount("/", handlers::user_routes())
        .mount("/", handlers::account_routes())
        .mount("/", handlers::group_routes())
        .mount("/", handlers::expense_routes())
        .mount("/", handlers::notification_routes())
        .mount("/", handlers::login_routes())
}
