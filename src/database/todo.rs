use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models::enums::importance::Importance;
use crate::schema::content_links;
use crate::schema::todos;

#[derive(Debug, Queryable)]
pub struct Todo {
    id: i32,
    timestamp: NaiveDateTime,
    end_at: NaiveDateTime,
    importance: i16,
    status: i16,
    text: String,
}

#[derive(Debug, Insertable)]
#[table_name = "todos"]
pub struct NewTodo {
    pub timestamp: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub importance: i16,
    pub status: i16,
    pub text: String,
}

#[derive(Debug, Insertable)]
#[table_name = "content_links"]
pub struct NewLink {
    pub link: Option<String>,
}

pub fn create(conn: &PgConnection, request: crate::models::todo::NewTodo) -> Todo {
    let new_todo = &NewTodo {
        timestamp: NaiveDateTime::from_timestamp(chrono::offset::Utc::now().timestamp(), 0),
        end_at: request.end_at,
        importance: request.importance as i16,
        status: request.status as i16,
        text: request.content.text,
    };

    diesel::insert_into(todos::table)
        .values(new_todo)
        .get_result(conn)
        .expect("Error creating todo")
}

pub fn get_all(conn: &PgConnection) -> Vec<Todo> {
    todos::table.load::<Todo>(conn).unwrap()
}

pub fn get_by_id(conn: &PgConnection, id: i32) -> Todo {
    todos::table.find(id).first(conn).unwrap()
}

pub fn delete(conn: &PgConnection, id: i32) -> usize {
    diesel::delete(todos::table)
        .filter(todos::id.eq(id))
        .execute(conn)
        .unwrap()
}

pub fn update_importance(conn: &PgConnection, id: i32, request: i16) -> Todo {
    diesel::update(todos::table.filter(todos::id.eq(id)))
        .set(todos::importance.eq(request))
        .get_result(conn)
        .unwrap()
}
