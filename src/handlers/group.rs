use mongodb::bson::{doc, oid::ObjectId};
use rocket::{serde::json::Json, State};

use crate::{
    config::AppConfig,
    models::entity::{Group, MemberUpdate},
};

// #[post("/groups", data = "<group>")]
// pub async fn create_group(
//     state: &State<AppConfig>,
//     group: Json<Group>,
// ) -> Result<Json<Group>, rocket::http::Status> {
//     let collection = state.mongodb.collection::<Group>("groups");

//     let new_group = Group {
//         id: None,
//         created_at: chrono::Utc::now().to_rfc3339(),
//         members: group.members,
//         admin_id: group.admin_id.clone(),
//         ..group.into_inner()
//     };

//     match collection.insert_one(&new_group).await {
//         Ok(_) => Ok(Json(new_group)),
//         Err(_) => Err(rocket::http::Status::InternalServerError),
//     }
// }

// #[post("/groups/<group_id>/expenses", data = "<expense>")]
// pub async fn add_group_expense(
//     state: &State<AppConfig>,
//     group_id: String,
//     expense: Json<Expense>,
// ) -> Result<Json<Expense>, rocket::http::Status> {
//     let collection = state.mongodb.collection::<Expense>("group_expenses");

//     let obj_id = ObjectId::parse_str(&group_id).map_err(|_| rocket::http::Status::BadRequest)?;

//     let new_expense = Expense {
//         id: None,
//         group_id: Some(obj_id),
//         participants: expense.participants.clone(),
//         ..expense.into_inner()
//     };

//     match collection.insert_one(&new_expense).await {
//         Ok(_) => Ok(Json(new_expense)),
//         Err(_) => Err(rocket::http::Status::InternalServerError),
//     }
// }

// #[get("/groups/<group_id>/expenses/history?<sort_by>")]
// pub async fn group_expense_history(
//     state: &State<AppConfig>,
//     group_id: String,
//     sort_by: Option<String>, // Options: "date", "amount", "category"
// ) -> Result<Json<Vec<Expense>>, rocket::http::Status> {
//     let collection = state.mongodb.collection::<Expense>("group_expenses");

//     let obj_id = ObjectId::parse_str(&group_id).map_err(|_| rocket::http::Status::BadRequest)?;

//     let mut pipeline = vec![doc! { "$match": { "group_id": obj_id } }];

//     if let Some(sort_field) = sort_by {
//         pipeline.push(doc! { "$sort": { sort_field: 1 } });
//     }

//     match collection.aggregate(pipeline).await {
//         Ok(cursor) => {
//             let expenses: Vec<Expense> = cursor.try_collect().await.unwrap();
//             Ok(Json(expenses))
//         }
//         Err(_) => Err(rocket::http::Status::InternalServerError),
//     }
// }

#[put("/groups/<group_id>/settlements/<user_id>")]
pub async fn settle_group_balance(
    state: &State<AppConfig>,
    group_id: String,
    user_id: String,
) -> Result<Json<bool>, rocket::http::Status> {
    let collection = state.mongodb.collection::<Group>("groups");

    let group_obj_id =
        ObjectId::parse_str(&group_id).map_err(|_| rocket::http::Status::BadRequest)?;
    let user_obj_id =
        ObjectId::parse_str(&user_id).map_err(|_| rocket::http::Status::BadRequest)?;

    let update = doc! {
        "$set": { "members.$[elem].settled": true }
    };

    let options = mongodb::options::UpdateOptions::builder()
        .array_filters(Some(vec![doc! { "elem.user_id": user_obj_id }]))
        .build();

    match collection
        .update_one(doc! { "_id": group_obj_id }, update)
        .with_options(options)
        .await
    {
        Ok(result) => {
            if result.matched_count > 0 {
                Ok(Json(true))
            } else {
                Err(rocket::http::Status::NotFound)
            }
        }
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}

#[put("/groups/<group_id>/members", data = "<member_update>")]
pub async fn update_group_members(
    state: &State<AppConfig>,
    group_id: String,
    member_update: Json<MemberUpdate>,
) -> Result<Json<bool>, rocket::http::Status> {
    let collection = state.mongodb.collection::<Group>("groups");

    let obj_id = ObjectId::parse_str(&group_id).map_err(|_| rocket::http::Status::BadRequest)?;

    let update_doc = match member_update.action.as_str() {
        "add" => doc! { "$push": { "members": member_update.member_id.clone() } },
        "remove" => doc! { "$pull": { "members": member_update.member_id.clone() } },
        _ => return Err(rocket::http::Status::BadRequest),
    };

    match collection
        .update_one(doc! { "_id": obj_id }, update_doc)
        .await
    {
        Ok(_) => Ok(Json(true)),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}
