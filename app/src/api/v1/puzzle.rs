use crate::db::models::Puzzle;
use crate::db::schema::puzzles::dsl::puzzles;
use crate::db::schema::puzzles::id;
use crate::db::ConnPool;
use actix_identity::{Identity};


use actix_web::{get, web, HttpResponse, Responder};
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
    pool: web::Data<ConnPool>,
    _identity: Identity,
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
            res
        }
        _ => HttpResponse::NotFound().body("Not found".to_string()),
    }
}
