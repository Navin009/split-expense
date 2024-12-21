use chrono::NaiveDateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub password: String, // Consider hashing passwords for security
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Expense {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub title: String,
    pub amount: f64,
    pub category: String,
    pub date: String,                // Store date as ISO 8601 string for simplicity
    pub receipt_url: Option<String>, // Optional URL for receipt
    pub comments: Option<String>,    // Optional comments
    pub is_group_expense: bool,      // True if it's a group-related expense
    pub participants: Vec<Participant>, // List of participants and their shares
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Participant {
    pub user_id: ObjectId,
    pub share: f64, // The share of the expense (e.g., 50.0 for 50%)
    pub is_paid: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub message: String,
    pub is_read: bool,
    pub created_at: String, // ISO 8601 timestamp
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // MongoDB ID

    pub name: String,        // Name of the group
    pub description: String, // Group description
    pub currency: String,    // Preferred currency for the group (e.g., "USD", "EUR")

    #[serde(rename = "admin_id")]
    pub admin_id: ObjectId, // Admin/creator of the group
    pub members: Vec<GroupMember>, // List of group members
    pub created_at: Option<NaiveDateTime>, // Timestamp of group creation
    pub updated_at: Option<NaiveDateTime>, // Timestamp of last update
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupMember {
    pub user_id: ObjectId, // MongoDB ID of the user
    pub name: String,      // User's name
    pub email: String,     // User's email
    #[serde(default)]
    pub settled: bool, // Whether the member has settled their balance
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberUpdate {
    pub action: String,      // Action type: "add" or "remove"
    pub member_id: ObjectId, // The ID of the member to add or remove
}
