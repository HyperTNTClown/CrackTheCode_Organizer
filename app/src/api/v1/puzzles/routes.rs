use actix_web::{get, HttpResponse, Responder, web};
use diesel::RunQueryDsl;
use crate::db::ConnPool;
use crate::db::models::Puzzle;

#[get("")]
async fn get_puzzles(pool: web::Data<ConnPool>) -> impl Responder {
    use crate::db::schema::puzzles::dsl::*;
    let mut conn = pool.get().unwrap();
    
    match puzzles.load::<Puzzle>(&mut conn) {
        Ok(lpuzzles) => HttpResponse::Ok().json(lpuzzles),
        Err(_) => HttpResponse::InternalServerError().json("Error connecting to database"),
    }
}