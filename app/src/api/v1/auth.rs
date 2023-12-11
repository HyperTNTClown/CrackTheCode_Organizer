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
use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, PooledConnection};
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

    let user = get_user_from_database(&mut conn, &json.email);
    match user {
        Ok(user) => {
            let hash = generate_password_hash(&json.password, &user.salt);
            if user.password.eq(&hash) {
                login_user(&req, &user, &session);
                HttpResponse::Ok().json("Logged in")
            } else {
                HttpResponse::Unauthorized().json("Unauthorized")
            }
        }
        _ => HttpResponse::Unauthorized().json("Unauthorized"),
    }
}

#[post("/auth/register")]
async fn register(
    pool: web::Data<ConnPool>,
    json: web::Json<UserRegisterRequest>,
) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let json = json.into_inner();
    if !json.email.ends_with("@deltalearns.ca") {
        return HttpResponse::Unauthorized()
            .json("Please sign up with your @deltalearns.ca email address");
    }

    let e = users.filter(email.eq(&json.email)).get_result::<User>(&mut conn);
    if e.is_ok() {
        HttpResponse::Conflict().json("User already exists")
    } else {
        let name = parse_username(&json.email);
        let new_user = NewUser::new(&name, &json.password, &json.email);
        diesel::insert_into(users)
            .values(&new_user)
            .execute(&mut conn)
            .expect("Error saving new user");
        HttpResponse::Ok().json(format!("User {name} created"))
    }
}

fn parse_username(address: &str) -> String {
    let name = &mut address.split('@').collect_vec()[0].chars();
    name.next_back();
    name.next_back();
    name.next_back();
    name.next_back();
    name.as_str().to_string()
}

#[get("/auth/valid")]
async fn valid(identity: Option<Identity>, session: Session) -> impl Responder {
    match identity {
        Some(e) => match session.get::<bool>("admin") {
            Ok(Some(_admin)) => HttpResponse::Ok().json(format!("Admin {}", e.id().unwrap())),
            _ => HttpResponse::Ok().json(format!("User {}", e.id().unwrap())),
        },
        _ => HttpResponse::Unauthorized().json("Unauthorized"),
    }
}

#[get("/auth/logout")]
async fn logout(identity: Identity) -> impl Responder {
    identity.logout();
    HttpResponse::Ok().json("Logged out")
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

fn get_user_from_database(conn: &mut PooledConnection<ConnectionManager<MysqlConnection>>, mail: &str) -> Result<User, diesel::result::Error> {
    users.filter(email.eq(mail)).first::<User>(conn)
}

fn generate_password_hash(password: &str, salt: &str) -> String {
    hex::encode(Hash::generate_hash(password, salt, "argon2i").unwrap())
}

fn login_user(req: &HttpRequest, user: &User, session: &Session) {
    Identity::login(&req.extensions(), user.email.to_string()).unwrap();
    if user.admin {
        session.insert("admin", true).unwrap();
    }
}