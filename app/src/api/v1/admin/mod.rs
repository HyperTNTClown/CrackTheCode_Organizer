mod routes;

use crate::api::v1::admin::routes::{add_member, create_puzzle, create_team, create_user, delete_member, delete_puzzle, delete_team, delete_user, get_all_members, get_all_puzzles, get_all_teams, get_all_users, get_puzzle, get_team, get_user, update_puzzle, update_team, update_user};
use crate::middleware::AuthenticatedRoutes;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .wrap(AuthenticatedRoutes::admin())
            .service(get_user)
            .service(get_all_users)
            .service(create_user)
            .service(delete_user)
            .service(update_user)
            .service(get_all_teams)
            .service(get_team)
            .service(create_team)
            .service(delete_team)
            .service(update_team)
            .service(get_all_members)
            .service(delete_member)
            .service(add_member)
            .service(get_puzzle)
            .service(get_all_puzzles)
            .service(create_puzzle)
            .service(delete_puzzle)
            .service(update_puzzle)
    );
}