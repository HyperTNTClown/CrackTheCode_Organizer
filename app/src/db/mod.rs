pub mod models;
pub mod schema;
pub(crate) mod users;
pub(crate) mod leaderboard;

use std::env;
use diesel::{Connection, MysqlConnection, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;

pub type ConnPool = Pool<ConnectionManager<MysqlConnection>>;
pub fn establish_connection() -> Pool<ConnectionManager<MysqlConnection>> {
    dotenv().ok();

    let mut database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = MysqlConnection::establish(&format!("{}/{}", &database_url, "mysql")).expect("Failed to connect to database");
    diesel::sql_query("CREATE DATABASE IF NOT EXISTS crackthecode").execute(&mut conn).expect("Failed to create test database");
    database_url.push_str("/crackthecode");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool.")
}

