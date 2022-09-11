use rocket::serde::json::{json, Json, Value};

use crate::database::todo;
use crate::database::{self, Db};
use crate::models::enums::importance::Importance;
use crate::models::{self, todo::NewTodo};

#[post("/create", format = "json", data = "<new_todo>")]
pub async fn create(new_todo: Json<NewTodo>, db: Db) {
    let todo = db
        .run(|conn| todo::create(conn, new_todo.into_inner()))
        .await;
    println!("{:?}", todo);
}

#[get("/all")]
pub async fn get_todos(db: Db) {
    let todos = db.run(|conn| todo::get_all(conn)).await;
    println!("{:#?}", todos);
}

#[get("/<id>")]
pub async fn get_todo(db: Db, id: i32) {
    let todo = db.run(move |conn| todo::get_by_id(conn, id)).await;
    println!("{:#?}", todo);
}

#[put("/imp/<id>/<importance>")]
pub async fn update_importance(db: Db, id: i32, importance: i16) {
    let updated = db
        .run(move |conn| todo::update_importance(conn, id, importance))
        .await;
    println!("{:#?}", updated);
}

#[delete("/<id>")]
pub async fn delete(db: Db, id: i32) {
    let result = db.run(move |conn| todo::delete(conn, id)).await;
    println!("{}", result);
}
