use diesel::{MysqlConnection};
use diesel::r2d2::{ConnectionManager, PooledConnection};

pub(crate) fn get_leaderboard(_conn: &mut PooledConnection<ConnectionManager<MysqlConnection>>) -> Option<String> {
    unimplemented!("get_leaderboard not implemented");
}