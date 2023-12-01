use actix_web::web::ServiceConfig;
use actix_web::{get, web, HttpResponse, Responder};

pub mod v1;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(v1::config)
            .service(hello)
            .service(ping),
    );
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body(format!("pong"))
}
