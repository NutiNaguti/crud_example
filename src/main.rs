#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_sync_db_pools;

mod config;
mod database;
mod models;
mod routes;
mod schema;

#[get("/")]
async fn index() -> &'static str {
    "Hello world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(database::Db::fairing())
        .mount("/", routes![index])
        .mount(
            "/api",
            routes![
                routes::crud::get_todos,
                routes::crud::create,
                routes::crud::get_todo,
                routes::crud::update_importance,
                routes::crud::delete
            ],
        )
}
