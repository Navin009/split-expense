use crate::models::entity::User;
use crate::AppConfig;
use mongodb::bson::{doc, oid::ObjectId};
use rocket::serde::json::Json;
use rocket::State;

#[put("/users/<id>/deactivate")]
pub async fn deactivate_account(
    state: &State<AppConfig>,
    id: String,
) -> Result<Json<bool>, rocket::http::Status> {
    let collection = state.mongodb.collection::<User>("users");
    let obj_id = ObjectId::parse_str(&id).map_err(|_| rocket::http::Status::BadRequest)?;

    let update_doc = doc! {
        "$set": { "deleted": true }
    };

    match collection
        .update_one(doc! { "_id": obj_id }, update_doc)
        .await
    {
        Ok(result) => {
            if result.matched_count > 0 {
                Ok(Json(true)) // Successfully deactivated
            } else {
                Err(rocket::http::Status::NotFound) // User not found
            }
        }
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}

#[put("/users/<id>/reactivate")]
pub async fn reactivate_account(
    state: &State<AppConfig>,
    id: String,
) -> Result<Json<bool>, rocket::http::Status> {
    let collection = state.mongodb.collection::<User>("users");
    let obj_id = ObjectId::parse_str(&id).map_err(|_| rocket::http::Status::BadRequest)?;

    let update_doc = doc! {
        "$set": { "deleted": false }
    };

    match collection
        .update_one(doc! { "_id": obj_id, "deleted": true }, update_doc)
        .await
    {
        Ok(result) => {
            if result.matched_count > 0 {
                Ok(Json(true)) // Successfully reactivated
            } else {
                Err(rocket::http::Status::NotFound) // User not found or not deactivated
            }
        }
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}
