use actix_web::{delete, get, HttpResponse, post, put, Responder};
use actix_web::web::{Data, Json, Path};
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use hsh::models::hash::Hash;
use crate::api::v1::auth::util::parse_username_from_email;
use crate::api::v1::models::UserRegisterRequest;
use crate::db::ConnPool;
use crate::db::models::{NewPuzzle, NewTeam, Puzzle, Team, User, UserTeamRef};
use crate::db::users::check_if_user_exists;


// Crud for users - GET, GET-all, POST, PUT, DELETE

#[get("/users/{user_id}")]
async fn get_user(pool: Data<ConnPool>, qid: Path<i32>) -> impl Responder {
    use crate::db::schema::users::id;
    use crate::db::models::User;
    use crate::db::schema::users::dsl::users;

    let Ok(mut conn) = pool.get()
        else {
            return HttpResponse::InternalServerError().json("Error connecting to database");
        };

    let user = users.filter(id.eq(qid.into_inner())).first::<User>(&mut conn);
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::NotFound().json("User not found"),
    }
}

#[get("/users")]
async fn get_all_users(pool: Data<ConnPool>) -> impl Responder {
    use crate::db::models::User;
    use crate::db::schema::users::dsl::users;

    let Ok(mut conn) = pool.get()
        else {
            return HttpResponse::InternalServerError().json("Error connecting to database");
        };

    let user = users.load::<User>(&mut conn);
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::NotFound().json("User not found"),
    }
}

#[post("/users")]
async fn create_user(pool: Data<ConnPool>, json: Json<UserRegisterRequest>) -> impl Responder {
    use crate::db::models::NewUser;
    use crate::db::schema::users::dsl::users;

    let Ok(mut conn) = pool.get()
        else {
            return HttpResponse::InternalServerError().json("Error connecting to database");
        };

    let json = json.into_inner();

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

#[put("/users/{user_id}")]
async fn update_user(pool: Data<ConnPool>, json: Json<User>, qid: Path<i32>) -> impl Responder {
    use crate::db::schema::users::dsl::users;
    use crate::db::schema::users::{id, name, email, password, salt};

    let Ok(mut conn) = pool.get()
        else {
            return HttpResponse::InternalServerError().json("Error connecting to database");
        };

    let json = json.into_inner();
    let lsalt = Hash::generate_salt("argon2i").unwrap();
    let hash = Hash::generate_hash(&json.password, &lsalt, "argon2i").unwrap();
    let hash = hex::encode(hash);
    match diesel::update(users.filter(id.eq(qid.into_inner())))
        .set((name.eq(json.name), email.eq(json.email), password.eq(hash), salt.eq(lsalt)))
        .execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().json("User updated"),
        _ => HttpResponse::NotFound().json("User not found"),
    }
}

#[delete("/users/{user_id}")]
async fn delete_user(pool: Data<ConnPool>, qid: Path<i32>) -> impl Responder {
    use crate::db::schema::users::dsl::users;
    use crate::db::schema::users::id;

    let Ok(mut conn) = pool.get()
        else {
            return HttpResponse::InternalServerError().json("Error connecting to database");
        };

    match diesel::delete(users.filter(id.eq(qid.into_inner()))).execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().json("User deleted"),
        _ => HttpResponse::NotFound().json("User not found"),
    }

    // TODO: Clean team associations etc.
    //      (Maybe also just schedule for deletion instead of actually deleting
    //      and then delete after a week or so,
    //      so that we can recover if we accidentally delete a user)
}

// Crud for Teams - GET, GET-all, POST, PUT, DELETE

#[get("/teams/{team_id}")]
async fn get_team(pool: Data<ConnPool>, qid: Path<i32>) -> impl Responder {
    use crate::db::schema::teams::dsl::teams;
    use crate::db::schema::teams::id;
    use crate::db::models::Team;
    let Ok(mut conn) = pool.get()
        else {
            return HttpResponse::InternalServerError().json("Error connecting to database");
        };

    match teams.filter(id.eq(qid.into_inner())).first::<Team>(&mut conn) {
        Ok(team) => HttpResponse::Ok().json(team),
        _ => HttpResponse::NotFound().json("Team not found"),
    }
}

#[get("/teams")]
async fn get_all_teams(pool: Data<ConnPool>) -> impl Responder {
    use crate::db::schema::teams::dsl::teams;
    use crate::db::models::Team;
    let Ok(mut conn) = pool.get()
        else {
            return HttpResponse::InternalServerError().json("Error connecting to database");
        };

    match teams.load::<Team>(&mut conn) {
        Ok(lteams) => HttpResponse::Ok().json(lteams),
        _ => HttpResponse::NotFound().json("Had trouble loading teams")
    }
}

#[post("/teams")]
async fn create_team(pool: Data<ConnPool>, json: Json<NewTeam>) -> impl Responder {
    use crate::db::schema::teams::dsl::teams;
    let Ok(mut conn) = pool.get()
        else {
            return HttpResponse::InternalServerError().json("Error connecting to database");
        };

    let json = json.into_inner();
    match diesel::insert_into(teams)
        .values(&json)
        .execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().json("Team created"),
        _ => HttpResponse::NotFound().json("Had trouble creating team")
    }
}

#[put("/teams/{team_id}")]
async fn update_team(pool: Data<ConnPool>, json: Json<Team>, qid: Path<i32>) -> impl Responder {
    use crate::db::schema::teams::dsl::teams;
    use crate::db::schema::teams::{id, name};

    let Ok(mut conn) = pool.get()
        else {
            return HttpResponse::InternalServerError().json("Error connecting to database");
        };

    let json = json.into_inner();
    match diesel::update(teams.filter(id.eq(qid.into_inner())))
        .set(name.eq(json.name))
        .execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().json("Team updated"),
        _ => HttpResponse::NotFound().json("Team not found"),
    }
}

#[delete("/teams/{team_id}")]
async fn delete_team(pool: Data<ConnPool>, qid: Path<i32>) -> impl Responder {
    use crate::db::schema::teams::dsl::teams;
    use crate::db::schema::teams::id;

    let Ok(mut conn) = pool.get()
        else {
            return HttpResponse::InternalServerError().json("Error connecting to database");
        };

    match diesel::delete(teams.filter(id.eq(qid.into_inner()))).execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().json("Team deleted"),
        _ => HttpResponse::NotFound().json("Team not found"),
    }

    // TODO: Clean team associations etc.
    //  | Schedule deletion... Maybe even in a different table
}

// Member for Teams - GET, GET-all, POST, PUT, DELETE

#[get("/teams/{team_id}/members")]
async fn get_all_members(pool: Data<ConnPool>, qid: Path<i32>) -> impl Responder {
    use crate::db::schema::user_teams::dsl::user_teams;
    use crate::db::schema::user_teams::team_id;
    use crate::db::models::UserTeamRef;

    let Ok(mut conn) = pool.get()
        else {
            return HttpResponse::InternalServerError().json("Error connecting to database");
        };

    match user_teams.filter(team_id.eq(qid.into_inner())).load::<UserTeamRef>(&mut conn) {
        Ok(lteams) => HttpResponse::Ok().json(lteams),
        _ => HttpResponse::NotFound().json("Had trouble loading teams")
    }
}

#[post("/teams/{team_id}/members")]
async fn add_member(pool: Data<ConnPool>, json: Json<UserTeamRef>, _path: Path<i32>) -> impl Responder {
    use crate::db::schema::user_teams::dsl::user_teams;
    use crate::db::schema::user_teams::user_id;

    let Ok(mut conn) = pool.get()
        else {
            return HttpResponse::InternalServerError().json("Error connecting to database");
        };

    let json = json.into_inner();
    // first check if user is already in a team, and remove them from that if they are
    diesel::delete(user_teams.filter(user_id.eq(json.user_id))).execute(&mut conn).unwrap();
    match diesel::insert_into(user_teams)
        .values(&json)
        .execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().json("Added member to team"),
        _ => HttpResponse::InternalServerError().json("Had trouble adding the user")
    }
}

#[delete("/teams/{team_id}/members/{member_id}")]
async fn delete_member(pool: Data<ConnPool>, qid: Path<(i32, i32)>) -> impl Responder {
    use crate::db::schema::user_teams::dsl::user_teams;
    use crate::db::schema::user_teams::{user_id, team_id};

    let Ok(mut conn) = pool.get()
        else {
            return HttpResponse::InternalServerError().json("Error connecting to database");
        };

    match diesel::delete(user_teams.filter(user_id.eq(qid.1).and(team_id.eq(qid.0)))).execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().json("Removed member from team"),
        _ => HttpResponse::InternalServerError().json("Had trouble removing the user")
    }
}


// Crud for Puzzles - GET, GET-all, POST, PUT, DELETE

#[get("/puzzles/{puzzle_id}")]
async fn get_puzzle(pool: Data<ConnPool>, qid: Path<i32>) -> impl Responder {
    use crate::db::schema::puzzles::dsl::puzzles;
    use crate::db::schema::puzzles::id;

    let Ok(mut conn) = pool.get()
        else {
            return HttpResponse::InternalServerError().json("Error connecting to database");
        };

    match puzzles.filter(id.eq(qid.into_inner())).first::<Puzzle>(&mut conn) {
        Ok(puzzle) => HttpResponse::Ok().json(puzzle),
        _ => HttpResponse::NotFound().json("Puzzle not found"),
    }
}

#[get("/puzzles")]
async fn get_all_puzzles(pool: Data<ConnPool>) -> impl Responder {
    use crate::db::schema::puzzles::dsl::puzzles;
    use crate::db::models::Puzzle;

    let Ok(mut conn) = pool.get()
        else {
            return HttpResponse::InternalServerError().json("Error connecting to database");
        };

    match puzzles.load::<Puzzle>(&mut conn) {
        Ok(lpuzzles) => HttpResponse::Ok().json(lpuzzles),
        _ => HttpResponse::NotFound().json("Puzzle not found"),
    }
}

#[post("/puzzles")]
async fn create_puzzle(pool: Data<ConnPool>, json: Json<NewPuzzle>) -> impl Responder {
    use crate::db::schema::puzzles::dsl::puzzles;

    let Ok(mut conn) = pool.get()
        else {
            return HttpResponse::InternalServerError().json("Error connecting to database");
        };

    let json = json.into_inner();
    match diesel::insert_into(puzzles)
        .values(&json)
        .execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().json("Puzzle created"),
        _ => HttpResponse::NotFound().json("Had trouble creating puzzle")
    }
}

#[put("/puzzles/{puzzle_id}")]
async fn update_puzzle(pool: Data<ConnPool>, json: Json<Puzzle>) -> impl Responder {
    use crate::db::schema::puzzles::dsl::puzzles;
    use crate::db::schema::puzzles::{id, name, description};

    let Ok(mut conn) = pool.get()
        else {
            return HttpResponse::InternalServerError().json("Error connecting to database");
        };

    let json = json.into_inner();
    match diesel::update(puzzles.filter(id.eq(json.id)))
        .set((name.eq(json.name), description.eq(json.description)))
        .execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().json("Puzzle updated"),
        _ => HttpResponse::NotFound().json("Puzzle not found"),
    }
}

#[delete("/puzzles/{puzzle_id}")]
async fn delete_puzzle(pool: Data<ConnPool>, qid: Path<i32>) -> impl Responder {
    use crate::db::schema::puzzles::dsl::puzzles;
    use crate::db::schema::puzzles::id;

    let Ok(mut conn) = pool.get()
        else {
            return HttpResponse::InternalServerError().json("Error connecting to database");
        };

    match diesel::delete(puzzles.filter(id.eq(qid.into_inner()))).execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().json("Puzzle deleted"),
        _ => HttpResponse::NotFound().json("Puzzle not found"),
    }
}

// Crud for Announcements - GET, GET-all, POST, PUT, DELETE

#[get("/announcements/{announcement_id}")]
async fn get_announcement(_pool: Data<ConnPool>) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/announcements")]
async fn get_all_announcements(_pool: Data<ConnPool>) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/announcements")]
async fn create_announcement(_pool: Data<ConnPool>) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[put("/announcements/{announcement_id}")]
async fn update_announcement(_pool: Data<ConnPool>) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[delete("/announcements/{announcement_id}")]
async fn delete_announcement(_pool: Data<ConnPool>) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[cfg(test)]
mod test {
    use super::*;
    use actix_web::{test, App};
    use actix_web::body::{BoxBody};
    use test::read_body_json;
    use crate::db::models::NewUser;
    use crate::test::get_test_pool;

    #[test]
    async fn test_get_user() {
        use crate::db::schema::users::dsl::users;
        use crate::db::schema::users::id;
        let test_pool = get_test_pool("test_get_user");
        let app = test::init_service(
            App::new()
                .service(get_user)
                .app_data(Data::new(test_pool.0.clone()))
        ).await;

        let req = test::TestRequest::get()
            .uri("/users/1")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());

        diesel::insert_into(users)
            .values(&NewUser::new("test", "test", "test@example.com"))
            .execute(&mut test_pool.0.get().unwrap())
            .unwrap();

        let req = test::TestRequest::get()
            .uri("/users/1")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let user: User = read_body_json::<User, BoxBody>(resp).await;

        assert_eq!(user.name, "test");
        assert_eq!(user.email, "test@example.com");

        diesel::delete(users.filter(id.eq(1))).execute(&mut test_pool.0.get().unwrap()).unwrap();

        let req = test::TestRequest::get()
            .uri("/users/1")
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(!resp.status().is_success());
    }

    #[test]
    async fn test_get_all_users() {
        use crate::db::schema::users::dsl::users;
        use crate::db::schema::users::id;
        let test_pool = get_test_pool("test_get_all_users");
        let app = test::init_service(
            App::new()
                .service(get_all_users)
                .app_data(Data::new(test_pool.0.clone()))
        ).await;

        diesel::insert_into(users)
            .values(&NewUser::new("test1", "test1", "test1@example.com"))
            .execute(&mut test_pool.0.get().unwrap())
            .unwrap();

        diesel::insert_into(users)
            .values(&NewUser::new("test2", "test2", "test2@example.com"))
            .execute(&mut test_pool.0.get().unwrap())
            .unwrap();

        let req = test::TestRequest::get()
            .uri("/users")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let users_l: Vec<User> = read_body_json::<Vec<User>, BoxBody>(resp).await;

        assert_eq!(users_l.len(), 2);

        diesel::delete(users.filter(id.eq(1))).execute(&mut test_pool.0.get().unwrap()).unwrap();
        diesel::delete(users.filter(id.eq(2))).execute(&mut test_pool.0.get().unwrap()).unwrap();
    }

    #[test]
    async fn test_create_user() {
        use crate::db::schema::users::dsl::users;
        use crate::db::schema::users::id;
        let test_pool = get_test_pool("test_create_user");
        let app = test::init_service(
            App::new()
                .service(create_user)
                .app_data(Data::new(test_pool.0.clone()))
        ).await;

        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(UserRegisterRequest {
                password: "test".parse().unwrap(),
                email: "testa324@deltalearns.ca".parse().unwrap(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let message: String = read_body_json::<String, BoxBody>(resp).await;

        assert_eq!(message, "User test created");

        diesel::delete(users.filter(id.eq(1))).execute(&mut test_pool.0.get().unwrap()).unwrap();
    }

    #[test]
    async fn test_create_user_conflict() {
        use crate::db::schema::users::dsl::users;
        use crate::db::schema::users::id;
        let test_pool = get_test_pool("test_create_user_conflict");
        let app = test::init_service(
            App::new()
                .service(create_user)
                .app_data(Data::new(test_pool.0.clone()))
        ).await;

        diesel::insert_into(users)
            .values(&NewUser::new("test", "test", "test@example.com"))
            .execute(&mut test_pool.0.get().unwrap())
            .unwrap();

        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(UserRegisterRequest {
                password: "test".parse().unwrap(),
                email: "test@example.com".parse().unwrap(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());

        diesel::delete(users.filter(id.eq(1))).execute(&mut test_pool.0.get().unwrap()).unwrap();
    }

    #[test]
    async fn test_get_team() {
        use crate::db::schema::teams::dsl::teams;
        use crate::db::schema::teams::id as id;

        let test_pool = get_test_pool("test_get_team");
        let app = test::init_service(
            App::new()
                .service(get_team)
                .app_data(Data::new(test_pool.0.clone()))
        ).await;

        diesel::insert_into(teams)
            .values(NewTeam { name: "test".parse().unwrap() })
            .execute(&mut test_pool.0.get().unwrap())
            .unwrap();

        let req = test::TestRequest::get()
            .uri("/teams/1")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let team: Team = read_body_json::<Team, BoxBody>(resp).await;

        assert_eq!(team.name, "test");

        diesel::delete(teams.filter(id.eq(1))).execute(&mut test_pool.0.get().unwrap()).unwrap();
    }

    #[test]
    async fn test_get_all_members() {
        use crate::db::schema::users::dsl::users;
        use crate::db::schema::users::id as u_user_id;
        use crate::db::schema::teams::dsl::teams;
        use crate::db::schema::teams::id as t_team_id;
        use crate::db::schema::user_teams::dsl::user_teams;
        use crate::db::schema::user_teams::user_id;
        let test_pool = get_test_pool("test_get_all_members");
        let app = test::init_service(
            App::new()
                .service(get_all_members)
                .app_data(Data::new(test_pool.0.clone()))
        ).await;

        diesel::insert_into(teams)
            .values(NewTeam { name: "test".parse().unwrap() })
            .execute(&mut test_pool.0.get().unwrap())
            .unwrap();

        diesel::insert_into(users)
            .values(&NewUser::new("test1", "test1", "test1@example.com"))
            .execute(&mut test_pool.0.get().unwrap())
            .unwrap();

        diesel::insert_into(users)
            .values(&NewUser::new("test2", "test2", "test2@example.com"))
            .execute(&mut test_pool.0.get().unwrap())
            .unwrap();

        diesel::insert_into(user_teams)
            .values(&UserTeamRef { user_id: 1, team_id: Some(1) })
            .execute(&mut test_pool.0.get().unwrap())
            .unwrap();

        diesel::insert_into(user_teams)
            .values(&UserTeamRef { user_id: 2, team_id: Some(1) })
            .execute(&mut test_pool.0.get().unwrap())
            .unwrap();

        let req = test::TestRequest::get()
            .uri("/teams/1/members")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let members: Vec<UserTeamRef> = read_body_json::<Vec<UserTeamRef>, BoxBody>(resp).await;

        assert_eq!(members.len(), 2);

        diesel::delete(user_teams.filter(user_id.eq(1).or(user_id.eq(2)))).execute(&mut test_pool.0.get().unwrap()).unwrap();
        diesel::delete(users.filter(u_user_id.eq(1).or(u_user_id.eq(2)))).execute(&mut test_pool.0.get().unwrap()).unwrap();
        diesel::delete(teams.filter(t_team_id.eq(1))).execute(&mut test_pool.0.get().unwrap()).unwrap();
    }
}
