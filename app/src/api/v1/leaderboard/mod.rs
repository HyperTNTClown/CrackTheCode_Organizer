mod routes;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/leaderboard")
            .service(routes::get_leaderboard)
    );
}