mod api;
mod db;
mod middleware;
use crate::db::establish_connection;
use actix_identity::IdentityMiddleware;
use actix_session::config::{CookieContentSecurity, PersistentSession, TtlExtensionPolicy};
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::{
    cookie::{time::Duration, Key},
    web, App, HttpServer,
};

#[cfg(test)]
mod test;

use diesel_migrations::{embed_migrations, EmbeddedMigrations};
pub const EMBEDDED_MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let pool = establish_connection();
    let key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(
                IdentityMiddleware::builder()
                    .visit_deadline(Some(core::time::Duration::from_secs(60 * 60)))
                    .build(),
            )
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), /* TODO: Change to Key::generate() to another one every time it restarts*/ key.clone())
                    .cookie_content_security(CookieContentSecurity::Private)
                    .cookie_secure(true)
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(Duration::hours(1))
                            .session_ttl_extension_policy(TtlExtensionPolicy::OnEveryRequest),
                    )
                    .build(),
            )
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(()))
            .configure(api::config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
