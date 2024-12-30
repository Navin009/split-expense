use log::error;
use mongodb::{
    bson::{doc, oid::ObjectId},
    results::UpdateResult,
};
use rocket::{serde::json::Json, State};

use crate::{config::AppConfig, guard::jwt_auth::JwtAuth, models::entity::User};

#[post("/user/v1/create", data = "<user>")]
pub async fn create_user(
    auth: JwtAuth,
    state: &State<AppConfig>,
    user: Json<User>,
) -> Result<Json<UpdateResult>, rocket::http::Status> {
    let collection = state.mongodb.collection::<User>("users");

    let upsert_doc = doc! {
        "name": &user.name,
        "password": &user.password,
        "created_at": chrono::Utc::now().to_rfc3339(),
    };

    match collection
        .update_one(doc! { "email": &user.email }, upsert_doc)
        .await
    {
        Ok(insert_result) => Ok(Json(insert_result)),
        Err(error) => {
            error!("Error creating user: {}", error);
            Err(rocket::http::Status::InternalServerError)
        }
    }
}

#[get("/user/v1/<id>")]
pub async fn get_user_profile(
    state: &State<AppConfig>,
    id: String,
) -> Result<Json<User>, rocket::http::Status> {
    let collection = state.mongodb.collection::<User>("users");
    let obj_id = ObjectId::parse_str(&id).map_err(|_| rocket::http::Status::BadRequest)?;

    match collection.find_one(doc! { "_id": obj_id }).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(rocket::http::Status::NotFound),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}

#[put("/user/v1/update?<id>", data = "<user>")]
pub async fn update_user_profile(
    state: &State<AppConfig>,
    id: String,
    user: Json<User>,
) -> Result<Json<UpdateResult>, rocket::http::Status> {
    let collection = state.mongodb.collection::<User>("users");
    let obj_id = ObjectId::parse_str(&id).map_err(|_| rocket::http::Status::BadRequest)?;

    let update_doc = doc! {
        "$set": {
            "name": &user.name,
            "email": &user.email,
        }
    };

    match collection
        .update_one(doc! { "_id": obj_id }, update_doc)
        .await
    {
        Ok(update_result) => Ok(Json(update_result)),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}
