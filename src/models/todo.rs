use chrono::naive::serde::ts_seconds;
use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

use super::content;
use super::enums;
use crate::database::todo;

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

impl From<NewTodo> for crate::database::todo::NewTodo {
    fn from(t: NewTodo) -> Self {
        todo::NewTodo {
            timestamp: NaiveDateTime::from_timestamp(0, 0),
            end_at: t.end_at,
            importance: t.importance as i16,
            status: t.status as i16,
            text: String::from(""),
        }
    }
}
