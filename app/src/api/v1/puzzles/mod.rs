use actix_web::web;
use actix_web::web::ServiceConfig;

mod routes;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/puzzles")
            .service(routes::get_puzzles)
    );
}
