use rocket::serde::json::{json, Json, Value};

use crate::models::{self, todo::NewTodo};

#[post("/create", format = "json", data = "<new_todo>")]
pub async fn create(new_todo: Json<NewTodo>) -> Json<NewTodo> {
    new_todo
}

#[get("/all")]
pub async fn get_todos() {}

#[get("/<id>")]
pub async fn get_todo(id: i32) {}

#[put("/<id>")]
pub async fn update(id: i32) {}

#[delete("/<id>")]
pub async fn delete(id: i32) {}
