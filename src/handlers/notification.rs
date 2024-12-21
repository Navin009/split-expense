use mongodb::bson::doc;
use rocket::{futures::TryStreamExt, serde::json::Json, State};

use crate::{config::AppConfig, models::entity::Notification};

#[post("/notifications", data = "<notification>")]
pub async fn add_notification(
    state: &State<AppConfig>,
    notification: Json<Notification>,
) -> Result<Json<Notification>, rocket::http::Status> {
    let collection = state.mongodb.collection::<Notification>("notifications");

    let new_notification = Notification {
        id: None,
        is_read: false,
        created_at: chrono::Utc::now().to_rfc3339(),
        ..notification.into_inner()
    };

    match collection.insert_one(&new_notification).await {
        Ok(_) => Ok(Json(new_notification)),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}

#[get("/notifications/<user_id>")]
pub async fn fetch_notifications(
    state: &State<AppConfig>,
    user_id: String,
) -> Result<Json<Vec<Notification>>, rocket::http::Status> {
    let collection = state.mongodb.collection::<Notification>("notifications");

    match collection.find(doc!{ "user_id": user_id }).await {
        Ok(cursor) => {
            let notifications: Vec<Notification> = cursor.try_collect().await.unwrap();
            Ok(Json(notifications))
        }
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}
