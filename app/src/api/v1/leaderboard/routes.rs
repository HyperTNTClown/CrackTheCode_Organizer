use actix_web::{get, HttpResponse, Responder};
use actix_web::web::Data;
use crate::db::ConnPool;

#[get("")]
async fn get_leaderboard(pool: Data<ConnPool>) -> impl Responder {
    use crate::db::leaderboard::get_leaderboard as get_leaderboard_db;

    let Ok(mut conn) = pool.get()
        else {
            return HttpResponse::InternalServerError().json("Error connecting to database");
        };

    let leaderboard = get_leaderboard_db(&mut conn);
    match leaderboard {
        Some(leaderboard) => HttpResponse::Ok().json(leaderboard),
        _ => HttpResponse::InternalServerError().json("Error getting leaderboard"),
    }
}