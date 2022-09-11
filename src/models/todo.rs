use chrono::naive::serde::ts_seconds;
use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

use super::content;
use super::enums;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Todo {
    pub id: i32,
    #[serde(with = "ts_seconds")]
    pub timestamp: NaiveDateTime,
    #[serde(with = "ts_seconds")]
    pub end_at: NaiveDateTime,
    pub importance: enums::importance::Importance,
    pub status: enums::status::Status,
    pub content: content::Content,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewTodo {
    #[serde(with = "ts_seconds")]
    pub end_at: NaiveDateTime,
    pub importance: enums::importance::Importance,
    pub status: enums::status::Status,
    pub content: content::Content,
}
