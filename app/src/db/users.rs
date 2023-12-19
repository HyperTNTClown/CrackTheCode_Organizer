use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, QueryResult, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::db::models::User;

pub fn get_user_by_email(conn: &mut PooledConnection<ConnectionManager<MysqlConnection>>, email: &str) -> QueryResult<User> {
    use crate::db::schema::users::dsl::{email as user_id, users};
    users.filter(user_id.eq(email)).first::<User>(conn)
}

pub fn check_if_user_exists(conn: &mut PooledConnection<ConnectionManager<MysqlConnection>>, email: &str) -> bool {
    use crate::db::schema::users::dsl::{email as user_id, users};
    let e = users.filter(user_id.eq(email)).first::<User>(conn);
    e.is_ok()
}