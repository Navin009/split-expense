use mongodb::bson::{doc, oid::ObjectId};
use rocket::{serde::json::Json, State};

use crate::{config::AppConfig, models::entity::Expense};

// #[get("/expenses?<category>&<start_date>&<end_date>")]
// pub async fn view_expenses(
//     state: &State<AppConfig>,
//     user_id: &String,
//     category: Option<String>,
//     start_date: Option<String>,
//     end_date: Option<String>,
// ) -> Result<Json<Vec<Expense>>, rocket::http::Status> {
//     let collection = state.mongodb.collection::<Expense>("expenses");
//     let mut filter = doc! { "user_id": user_id };

//     if let Some(cat) = category {
//         filter.insert("category", cat);
//     }

//     if let Some(start) = start_date {
//         filter.insert("date", doc! { "$gte": start });
//     }

//     if let Some(end) = end_date {
//         filter.insert("date", doc! { "$lte": end });
//     }

//     match collection.find(filter).await {
//         Ok(cursor) => {
//             let expenses: Vec<Expense> = cursor.try_collect().await.unwrap();
//             Ok(Json(expenses))
//         }
//         Err(_) => Err(rocket::http::Status::InternalServerError),
//     }
// }

#[post("/expenses", data = "<expense>")]
pub async fn add_expense(
    state: &State<AppConfig>,
    expense: Json<Expense>,
) -> Result<Json<Expense>, rocket::http::Status> {
    let collection = state.mongodb.collection::<Expense>("expenses");

    let new_expense = Expense {
        id: None,
        ..expense.into_inner()
    };

    match collection.insert_one(&new_expense).await {
        Ok(_) => Ok(Json(new_expense)),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}

#[put("/expenses/<id>", data = "<expense>")]
pub async fn edit_expense(
    state: &State<AppConfig>,
    id: String,
    expense: Json<Expense>,
) -> Result<Json<Expense>, rocket::http::Status> {
    let collection = state.mongodb.collection::<Expense>("expenses");
    let obj_id = ObjectId::parse_str(&id).map_err(|_| rocket::http::Status::BadRequest)?;

    let update_doc = doc! {
        "$set": {
            "title": expense.title.clone(),
            "amount": expense.amount,
            "category": expense.category.clone(),
            "date": expense.date.clone(),
            "receipt_url": expense.receipt_url.clone(),
            "comments": expense.comments.clone(),
        }
    };

    match collection
        .update_one(doc! { "_id": obj_id }, update_doc)
        .await
    {
        Ok(_) => Ok(expense),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}

#[delete("/expenses/<id>")]
pub async fn delete_expense(
    state: &State<AppConfig>,
    id: String,
) -> Result<Json<bool>, rocket::http::Status> {
    let collection = state.mongodb.collection::<Expense>("expenses");
    let obj_id = ObjectId::parse_str(&id).map_err(|_| rocket::http::Status::BadRequest)?;

    match collection.delete_one(doc! { "_id": obj_id }).await {
        Ok(result) => Ok(Json(result.deleted_count > 0)),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}

// #[get("/expenses/summary/<user_id>?<year>&<month>")]
// pub async fn expense_summary(
//     state: &State<AppConfig>,
//     user_id: String,
//     year: Option<String>,
//     month: Option<String>,
// ) -> Result<Json<serde_json::Value>, rocket::http::Status> {
//     let collection = state.mongodb.collection::<Expense>("expenses");

//     let mut filter = doc! { "user_id": user_id };
//     if let Some(y) = year {
//         filter.insert("date", doc! { "$regex": format!(r"^{}-", y) });
//     }
//     if let Some(m) = month {
//         filter.insert(
//             "date",
//             doc! { "$regex": format!(r"-{:02}-", m.parse::<u8>().unwrap()) },
//         );
//     }

//     let pipeline = vec![
//         doc! { "$match": filter },
//         doc! { "$group": {
//             "_id": null,
//             "total_expenses": { "$sum": "$amount" },
//         }},
//     ];

//     match collection.aggregate(pipeline).await {
//         Ok(mut cursor) => {
//             let summary: Vec<mongodb::bson::Document> =
//                 cursor.try_collect().await.unwrap_or_default();
//             if let Some(first_summary) = summary.first() {
//                 Ok(Json(Expense::parse_summary(first_summary.clone())))
//             } else {
//                 Ok(Json(doc! {}))
//             }
//         }
//         Err(_) => Err(rocket::http::Status::InternalServerError),
//     }
// }

// #[get("/expenses/overview?<category>&<date_range>&<group>")]
// pub async fn expense_overview(
//     state: &State<AppConfig>,
//     user_id: String,
//     category: Option<String>,
//     date_range: Option<String>, // e.g., "2024-12-01 to 2024-12-20"
//     group: Option<String>,
// ) -> Result<Json<Vec<Expense>>, rocket::http::Status> {
//     let collection = state.mongodb.collection::<Expense>("expenses");

//     let mut filter = doc! { "user_id": user_id };
//     if let Some(cat) = category {
//         filter.insert("category", cat);
//     }
//     if let Some(range) = date_range {
//         let dates: Vec<&str> = range.split(" to ").collect();
//         if dates.len() == 2 {
//             filter.insert("date", doc! { "$gte": dates[0], "$lte": dates[1] });
//         }
//     }
//     if let Some(group_id) = group {
//         filter.insert("group_id", group_id);
//     }

//     match collection.find(filter).await {
//         Ok(cursor) => {
//             let expenses: Vec<Expense> = cursor.try_collect().await.unwrap();
//             Ok(Json(expenses))
//         }
//         Err(_) => Err(rocket::http::Status::InternalServerError),
//     }
// }

#[post("/expenses/split", data = "<expense>")]
pub async fn split_expense(
    state: &State<AppConfig>,
    expense: Json<Expense>,
) -> Result<Json<Expense>, rocket::http::Status> {
    let collection = state.mongodb.collection::<Expense>("expenses");

    let total_share: f64 = expense.participants.iter().map(|p| p.share).sum();

    if (total_share - 100.0).abs() > f64::EPSILON {
        return Err(rocket::http::Status::BadRequest); // Ensure shares sum to 100%
    }

    let new_expense = Expense {
        id: None,
        ..expense.into_inner()
    };

    match collection.insert_one(&new_expense).await {
        Ok(_) => Ok(Json(new_expense)),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}

#[put("/expenses/<expense_id>/settle/<participant_id>")]
pub async fn settle_balance(
    state: &State<AppConfig>,
    expense_id: String,
    participant_id: String,
) -> Result<Json<bool>, rocket::http::Status> {
    let collection = state.mongodb.collection::<Expense>("expenses");

    let expense_obj_id =
        ObjectId::parse_str(&expense_id).map_err(|_| rocket::http::Status::BadRequest)?;
    let participant_obj_id =
        ObjectId::parse_str(&participant_id).map_err(|_| rocket::http::Status::BadRequest)?;

    let update = doc! {
        "$set": { "participants.$[elem].is_paid": true }
    };

    let options = mongodb::options::UpdateOptions::builder()
        .array_filters(Some(vec![doc! { "elem.user_id": participant_obj_id }]))
        .build();

    match collection
        .update_one(doc! { "_id": expense_obj_id }, update)
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

// #[get("/expenses/report?<format>")]
// pub async fn export_expense_report(
//     state: &State<AppConfig>,
//     user_id: String,
//     format: Option<String>, // "pdf", "excel", "csv"
// ) -> Result<Json<String>, rocket::http::Status> {
//     let collection = state.mongodb.collection::<Expense>("expenses");

//     let filter = doc! { "user_id": user_id };
//     match collection.find(filter).await {
//         Ok(cursor) => {
//             let expenses: Vec<Expense> = cursor.try_collect().await.unwrap();

//             let report = match format.unwrap_or_else(|| "csv".to_string()).as_str() {
//                 "pdf" => generate_pdf_report(&expenses),
//                 "excel" => generate_excel_report(&expenses),
//                 "csv" => generate_csv_report(&expenses),
//                 _ => return Err(rocket::http::Status::BadRequest),
//             };

//             Ok(Json(report))
//         }
//         Err(_) => Err(rocket::http::Status::InternalServerError),
//     }
// }
