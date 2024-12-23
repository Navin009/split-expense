pub mod account;
pub mod expense;
pub mod group;
pub mod login;
pub mod notification;
pub mod ping;
pub mod user;

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

pub fn expense_routes() -> Vec<rocket::Route> {
    routes![
        expense::add_expense,
        expense::edit_expense,
        expense::delete_expense,
        expense::settle_balance,
        expense::split_expense
    ]
}

pub fn notification_routes() -> Vec<rocket::Route> {
    routes![
        notification::add_notification,
        notification::fetch_notifications
    ]
}

pub fn group_routes() -> Vec<rocket::Route> {
    routes![group::settle_group_balance, group::update_group_members]
}

pub fn login_routes() -> Vec<rocket::Route> {
    routes![login::login]
}
