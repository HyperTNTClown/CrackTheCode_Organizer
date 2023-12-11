use crate::db::models::{NewPuzzle, NewPuzzleSet, PuzzleSet};
use crate::db::schema::puzzle_set::dsl::puzzle_set;
use crate::db::schema::puzzles::dsl::puzzles;
use crate::db::schema::puzzles::{description, id, name};
use crate::db::ConnPool;
use actix_identity::{Identity, IdentityExt};
use actix_session::SessionExt;
use actix_web::{get, guard, patch, put, web, HttpResponse, Responder};
use actix_web::web::service;

use diesel::{
    ExpressionMethods, QueryDsl, RunQueryDsl,
};
use serde::{Deserialize};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .service(fetch)
            .guard(guard::fn_guard(|req| {
                return true;
                let identity = req.get_identity();
                if identity.is_err() {
                    return false;
                }
                let session = req.get_session();
                if let Ok(Some(admin)) = session.get::<bool>("admin") {
                    return admin;
                }
                false
            }))
            .service(get_puzzule)
            .service(create_puzzle)
            .service(update_puzzle)
            .service(create_puzzle_set)
    );
}

#[derive(Deserialize)]
struct FetchQuery {
    puzzle_set_id: Option<i32>,
}

#[get("/fetch")]
async fn fetch(
    pool: web::Data<ConnPool>,
    identity: Identity,
    query: web::Query<FetchQuery>,
) -> impl Responder {
    if query.puzzle_set_id.is_none() {
        let mut conn = pool.get().unwrap();
        let sets = puzzle_set.load::<PuzzleSet>(&mut conn).unwrap();
        return HttpResponse::Ok().json(sets);
    }
    HttpResponse::Ok().body(format!("Hello, {}!", identity.id().unwrap()))
}

#[get("/puzzle/{puzzle_id}")]
async fn get_puzzule(
    _pool: web::Data<ConnPool>,
    identity: Identity,
    path: web::Path<i32>,
) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Hello, {}! Puzzle {}",
        identity.id().unwrap(),
        path.into_inner()
    ))
}

#[patch("/puzzle/{puzzle_id}")]
async fn update_puzzle(
    pool: web::Data<ConnPool>,
    identity: Identity,
    path: web::Path<i32>,
    json: web::Json<NewPuzzle>,
) -> impl Responder {
    let mut conn = pool.get().unwrap();

    diesel::update(puzzles.filter(id.eq(path.into_inner())))
        .set((name.eq(json.0.name), description.eq(json.0.description)))
        .execute(&mut conn)
        .unwrap();

    HttpResponse::Ok().body(format!("Hello, {}!", identity.id().unwrap()))
}

#[put("/puzzle")]
async fn create_puzzle(
    pool: web::Data<ConnPool>,
    identity: Identity,
    json: web::Json<NewPuzzle>,
) -> impl Responder {
    let mut conn = pool.get().unwrap();

    diesel::insert_into(puzzles)
        .values(json.0)
        .execute(&mut conn)
        .unwrap();

    HttpResponse::Ok().body(format!("Hello, {}!", identity.id().unwrap()))
}

#[put("/puzzle-set")]
async fn create_puzzle_set(
    pool: web::Data<ConnPool>,
    identity: Identity,
    json: web::Json<NewPuzzleSet>,
) -> impl Responder {
    use crate::db::schema::puzzle_set::dsl::name;
    let mut conn = pool.get().unwrap();

    diesel::insert_into(puzzle_set)
        .values(&json.0)
        .execute(&mut conn)
        .unwrap();

    let e = puzzle_set.filter(name.eq(&json.0.name)).first::<PuzzleSet>(&mut conn).unwrap();
    HttpResponse::Ok().json(e)
}