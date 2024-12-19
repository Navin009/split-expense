pub mod ping;

pub fn ping_routes() -> Vec<rocket::Route> {
    routes![
        ping::ping,
        ping::health,
        ping::db_check,
        ping::metrics,
        ping::prometheus
    ]
}
