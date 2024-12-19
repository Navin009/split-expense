pub mod ping;

pub fn ping_routes() -> Vec<rocket::Route> {
    routes![ping::index, ping::get_data]
}
