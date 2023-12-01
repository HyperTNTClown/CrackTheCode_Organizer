use crate::db::models::Puzzle;
use crate::db::schema::puzzles::dsl::puzzles;
use crate::db::schema::puzzles::id;
use crate::db::DbPool;
use actix_identity::{Identity, IdentityExt};
use actix_session::SessionExt;
use actix_web::http::header::HeaderValue;
use actix_web::{get, guard, web, HttpResponse, Responder};
use diesel::prelude::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    /*.guard(guard::fn_guard(|req| {
        let identity = req.get_identity();
        if identity.is_err() {
            return false;
        }
        return true;
    }))*/
    cfg.service(puzzle_markdown);
}

#[get("/puzzle/{puzzle_id}.md")]
async fn puzzle_markdown(
    pool: web::Data<DbPool>,
    identity: Identity,
    path: web::Path<i32>,
) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let puzzle = puzzles
        .filter(id.eq(path.into_inner()))
        .first::<Puzzle>(&mut conn);
    match puzzle {
        Ok(puzzle) => {
            let markdown = puzzle.description;
            let mut res = HttpResponse::Ok().body(markdown);
            res.headers_mut().append(
                "Content-Type".parse().unwrap(),
                "text/markdown; charset=utf-8".parse().unwrap(),
            );
            return res;
        }
        _ => HttpResponse::NotFound().body(format!("Not found")),
    }
}
