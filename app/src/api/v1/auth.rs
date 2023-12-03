use crate::api::v1::models::{
    IsAdminRequest, IsAdminResponse, UserLoginRequest, UserRegisterRequest,
};
use crate::db::models::{NewUser, User};
use crate::db::schema::users::dsl::users;
use crate::db::schema::users::email;
use crate::db::ConnPool;
use actix_identity::Identity;
use actix_session::Session;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use hsh::models::hash::Hash;
use itertools::Itertools;
use serde::Serialize;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login)
        .service(register)
        .service(valid)
        .service(logout)
        .service(is_admin)
        .service(valid_admin);
}

#[post("/auth/login")]
async fn login(
    pool: web::Data<ConnPool>,
    req: HttpRequest,
    json: web::Json<UserLoginRequest>,
    session: Session,
) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let json = json.into_inner();

    let user = users.filter(email.eq(&json.email)).first::<User>(&mut conn);
    match user {
        Ok(user) => {
            let salt = &user.salt;
            let hash = hex::encode(Hash::generate_hash(&json.password, salt, "argon2i").unwrap());
            if user.password.eq(&hash) {
                Identity::login(&req.extensions(), user.email).unwrap();
                if user.admin {
                    session.insert("admin", true).unwrap();
                }
                HttpResponse::Ok().json("Logged in".to_string())
            } else {
                HttpResponse::Unauthorized().body("Unauthorized".to_string())
            }
        }
        _ => HttpResponse::Unauthorized().body("Unauthorized".to_string()),
    }
}

#[post("/auth/register")]
async fn register(pool: web::Data<ConnPool>, json: web::Json<UserRegisterRequest>) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let json = json.into_inner();
    if !json.email.ends_with("@deltalearns.ca") {
        return HttpResponse::Unauthorized().body("Please sign up with your @deltalearns.ca email address".to_string());
    }

    let e = users.filter(email.eq(&json.email)).first::<User>(&mut conn);
    if e.is_ok() { HttpResponse::Conflict().body("User already exists".to_string()) } else {
        let name = &mut json.email.split('@').collect_vec()[0].chars();
        name.next_back();
        name.next_back();
        name.next_back();
        name.next_back();
        let name = name.as_str().to_string();
        let new_user = NewUser::new(&name, &json.password, &json.email);
        diesel::insert_into(users)
            .values(&new_user)
            .execute(&mut conn)
            .expect("TODO: panic message");
        HttpResponse::Ok().body(format!("User {name} created"))
    }
}

#[get("/auth/valid")]
async fn valid(identity: Option<Identity>, session: Session) -> impl Responder {
    match identity {
        Some(e) => match session.get::<bool>("admin") {
            Ok(Some(_admin)) => HttpResponse::Ok().body(format!("Admin {}", e.id().unwrap())),
            _ => HttpResponse::Ok().body(format!("User {}", e.id().unwrap())),
        },
        _ => HttpResponse::Unauthorized().body("Unauthorized".to_string()),
    }
}

#[get("/auth/logout")]
async fn logout(identity: Identity) -> impl Responder {
    identity.logout();
    HttpResponse::Ok().body("Logged out".to_string())
}

#[get("/is-admin")]
async fn is_admin(q: web::Query<IsAdminRequest>, pool: web::Data<ConnPool>) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let e = users.filter(email.eq(&q.email)).first::<User>(&mut conn);

    match e {
        Ok(e) => {
            if e.admin {
                HttpResponse::Ok().json(IsAdminResponse { is_admin: true })
            } else {
                HttpResponse::Ok().json(IsAdminResponse { is_admin: false })
            }
        }
        _ => HttpResponse::Ok().json(IsAdminResponse { is_admin: false }),
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

#[get("/auth/valid-admin")]
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
