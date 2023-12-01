pub mod models;
pub mod schema;

use std::env;
use diesel::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;
pub fn establish_connection() -> Pool<ConnectionManager<MysqlConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool.")
}