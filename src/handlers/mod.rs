pub mod account;
pub mod expense;
pub mod ping;
pub mod user;
pub mod notification;
pub mod group;

pub fn ping_routes() -> Vec<rocket::Route> {
    routes![
        ping::ping,
        ping::health,
        ping::db_check,
        ping::metrics,
        ping::prometheus
    ]
}

pub fn user_routes() -> Vec<rocket::Route> {
    routes![
        user::get_user_profile,
        user::create_user,
        user::update_user_profile
    ]
}

pub fn account_routes() -> Vec<rocket::Route> {
    routes![account::deactivate_account, account::reactivate_account]
}
