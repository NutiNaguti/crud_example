use rocket::serde::json::{json, Json, Value};

use crate::database::todo;
use crate::database::{self, Db};
use crate::models::{self, todo::NewTodo};

#[post("/create", format = "json", data = "<new_todo>")]
pub async fn create(new_todo: Json<NewTodo>, db: Db) {
    let todo = db
        .run(|conn| todo::create(conn, new_todo.into_inner()))
        .await;
    println!("{:?}", todo);
}

#[get("/all")]
pub async fn get_todos() {}

#[get("/<id>")]
pub async fn get_todo(id: i32) {}

#[put("/<id>")]
pub async fn update(id: i32) {}

#[delete("/<id>")]
pub async fn delete(id: i32) {}
