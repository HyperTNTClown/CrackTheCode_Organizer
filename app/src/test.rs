use diesel::{Connection, MysqlConnection, r2d2, RunQueryDsl};
use diesel::r2d2::ConnectionManager;
use diesel_migrations::MigrationHarness;
use dotenvy::dotenv;

use crate::db::ConnPool;
use crate::EMBEDDED_MIGRATIONS;

pub struct TestPool(pub ConnPool, String);

impl Drop for TestPool {
    fn drop(&mut self) {
        let mut conn = self.0.get().expect("Failed to get db connection");
        diesel::sql_query(format!("DROP DATABASE {}", self.1)).execute(&mut conn).expect("Failed to drop test database");
    }
}

pub fn get_test_pool(database_name: &str) -> TestPool {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("Database url not set");

    let mut conn = MysqlConnection::establish(&format!("{}/{}", &db_url, "mysql")).expect("Failed to connect to database");
    diesel::sql_query(format!("CREATE DATABASE IF NOT EXISTS {database_name}")).execute(&mut conn).expect("Failed to create test database");
    let manager = ConnectionManager::<MysqlConnection>::new(&format!("{}/{}", &db_url, database_name));
    let pool_size = if cfg!(test) { 1 } else { 10 };
    let pool = r2d2::Builder::new().max_size(pool_size).build(manager).expect("Failed to create db pool");

    let mut conn = pool.get().expect("Failed to get db connection");

    match conn.run_pending_migrations(EMBEDDED_MIGRATIONS) {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to run migrations: {e}");
        }
    }

    TestPool(pool, database_name.to_owned())
}