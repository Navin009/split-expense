use mongodb::bson::doc;
use rocket::{http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};

use crate::{config::AppConfig, models::entity::User, security::jwt::JWTAuthenticator};

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[post("/login", data = "<req_data>")]
pub async fn login(
    auth: &State<JWTAuthenticator>,
    req_data: Json<LoginForm>,
    state: &State<AppConfig>,
) -> Result<Json<String>, Status> {
    let collection = state.mongodb.collection::<User>("users");

    let user = collection.find_one(doc! { "email": &req_data.email }).await;

    let user = match user {
        Ok(user) => user,
        Err(_) => return Err(Status::InternalServerError),
    };

    match user {
        Some(user) => {
            if user.password == req_data.password {
                let token = auth.create_jwt(user.email.as_str());

                return Ok(Json(token));
            } else {
                Err(Status::Unauthorized)
            }
        }
        None => Err(Status::Unauthorized),
    }
}
