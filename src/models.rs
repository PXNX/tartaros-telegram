use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct Account {
    pub id: i32,
    pub api_key: String,
    pub valid_until: DateTime<Utc>,
}


#[derive(Serialize)]
pub struct Report {
    pub id: i32,
    pub message: String,
    pub user_id: i64,
    pub account_id: i32,
    pub reported_at: DateTime<Utc>,
    pub is_banned: Option<bool>,

}

pub struct NewReport {
    pub message: String,
    pub user_id: i64,
    pub account_id: i32,
}

#[derive(Deserialize, Debug)]
pub struct InputReport {
    pub message: String,
    pub user_id: i64,
}

pub struct User {
    pub id: i64,
    pub banned_since: DateTime<Utc>,
    pub messages: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    pub details: String,
}