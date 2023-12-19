use actix_identity::Identity;
use actix_session::Session;
use actix_web::{get, HttpRequest, HttpResponse, post, Responder, web};
use actix_web::web::Data;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use web::Json;
use crate::api::v1::auth::util::{generate_password_hash, login_user, parse_username_from_email};
use crate::api::v1::models::{IsAdminRequest, IsAdminResponse, UserLoginRequest, UserRegisterRequest};
use crate::db::ConnPool;
use crate::db::models::{NewUser, User};
use crate::db::schema::users::dsl::users;
use crate::db::schema::users::email;
use crate::db::users::{check_if_user_exists, get_user_by_email};

#[post("/login")]
async fn login(
    pool: Data<ConnPool>,
    req: HttpRequest,
    json: Json<UserLoginRequest>,
    session: Session,
) -> impl Responder {
    let Ok(mut conn) = pool.get()
        else {
            return HttpResponse::InternalServerError().json("Error connecting to database");
        };
    let json = json.into_inner();

    let user = get_user_by_email(&mut conn, &json.email);
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

#[post("/register")]
async fn register(
    pool: Data<ConnPool>,
    json: Json<UserRegisterRequest>,
) -> impl Responder {
    let Ok(mut conn) = pool.get()
        else {
            return HttpResponse::InternalServerError().json("Error connecting to database");
        };

    let json = json.into_inner();
    if !json.email.ends_with("@deltalearns.ca") {
        return HttpResponse::Unauthorized()
            .json("Please sign up with your @deltalearns.ca email address");
    }

    if check_if_user_exists(&mut conn, &json.email) {
        HttpResponse::Conflict().json("User already exists")
    } else {
        let name = parse_username_from_email(&json.email);
        let new_user = NewUser::new(&name, &json.password, &json.email);
        diesel::insert_into(users)
            .values(&new_user)
            .execute(&mut conn)
            .expect("Error saving new user");
        HttpResponse::Ok().json(format!("User {name} created"))
    }
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

#[get("/me")]
async fn me(identity: actix_identity::Identity, pool: Data<ConnPool>) -> impl Responder {
    let Ok(mut conn) = pool.get()
        else {
            return HttpResponse::InternalServerError().json("Error connecting to database");
        };
    let mut e = get_user_by_email(&mut conn, &identity.id().unwrap()).unwrap();
    e.salt = String::new();
    e.password = String::new();
    HttpResponse::Ok().json(e)
}

#[get("/logout")]
async fn logout(identity: Identity) -> impl Responder {
    identity.logout();
    HttpResponse::Ok().json("Logged out")
}
