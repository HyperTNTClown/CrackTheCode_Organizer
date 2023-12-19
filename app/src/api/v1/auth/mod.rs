mod routes;
pub mod util;

use crate::api::v1::auth::routes::{is_admin, login, logout, me, register};

use actix_identity::Identity;
use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(login)
            .service(register)
            .service(is_admin)
            .service(logout)
            .service(me)
            .service(valid)
            .service(valid_admin)
    );
}

#[get("/valid")]
async fn valid(identity: Option<Identity>, session: Session) -> impl Responder {
    match identity {
        Some(e) => match session.get::<bool>("admin") {
            Ok(Some(_admin)) => HttpResponse::Ok().json(format!("Admin {}", e.id().unwrap())),
            _ => HttpResponse::Ok().json(format!("User {}", e.id().unwrap())),
        },
        _ => HttpResponse::Unauthorized().json("Unauthorized"),
    }
}

#[derive(Serialize)]
struct ValidAdminResponse {
    valid: bool,
    pub id: Option<String>,
}

impl ValidAdminResponse {
    pub fn true_response(id: Option<String>) -> Self {
        Self { valid: true, id }
    }
    pub fn false_response(id: Option<String>) -> Self {
        Self { valid: false, id }
    }
}

#[get("/valid-admin")]
async fn valid_admin(identity: Option<Identity>, session: Session) -> impl Responder {
    match identity {
        Some(e) => match session.get::<bool>("admin") {
            Ok(Some(_)) => {
                HttpResponse::Ok().json(ValidAdminResponse::true_response(Some(e.id().unwrap())))
            }
            _ => HttpResponse::Ok().json(ValidAdminResponse::false_response(Some(e.id().unwrap()))),
        },
        _ => HttpResponse::Ok().json(ValidAdminResponse::false_response(None)),
    }
}