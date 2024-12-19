use rocket::State;

use crate::config::AppConfig;

#[get("/")]
pub async fn index() -> &'static str {
    "Welcome to Splitwise!"
}

#[get("/data")]
pub async fn get_data(state: &State<AppConfig>) -> String {
    let client = &state.mongo_client;

    // Example MongoDB query: Fetch a list of collections in the DB
    let db = client.database("splitwise");
    let collections = db.list_collection_names().await.unwrap();

    format!("MongoDB Collections: {:?}", collections)
}
