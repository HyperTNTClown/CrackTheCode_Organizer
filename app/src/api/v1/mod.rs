use actix_web::web;
use actix_web::web::ServiceConfig;

mod admin;
mod auth;
mod models;
mod puzzle;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .configure(
                admin::config
            )
            .configure(
                auth::config
            )
            .configure(
                puzzle::config
            )
    );
}
