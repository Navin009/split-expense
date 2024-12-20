#[macro_use]
extern crate rocket;

use crate::config::AppConfig;
use log::info;
use middleware::logging::LoggerFairing;
use rocket::{Build, Rocket};

mod config;
mod handlers;
mod middleware;
mod guard;

#[launch]
fn rocket() -> Rocket<Build> {
    AppConfig::init_logger();

    let app_config = rocket::tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(AppConfig::new())
        .unwrap();

    info!("Rocket app is launching");

    rocket::build()
        .manage(app_config)
        .attach(LoggerFairing)
        .mount("/", handlers::ping_routes())
}
