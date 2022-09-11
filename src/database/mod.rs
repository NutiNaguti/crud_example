use rocket_sync_db_pools::database;

pub mod todo;

#[database("todo_db")]
pub struct Db(diesel::PgConnection);
