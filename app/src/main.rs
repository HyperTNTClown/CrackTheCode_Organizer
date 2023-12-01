mod api;
mod db;

use crate::db::establish_connection;
use actix_identity::IdentityMiddleware;
use actix_session::config::{CookieContentSecurity, PersistentSession, TtlExtensionPolicy};
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::{
    cookie::{time::Duration, Key},
    web, App, HttpServer,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let pool = establish_connection();

    HttpServer::new(move || {
        App::new()
            .wrap(
                IdentityMiddleware::builder()
                    .visit_deadline(Some(core::time::Duration::from_secs(60 * 60)))
                    .build(),
            )
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), /* TODO: Change to Key::generate() to another one every time it restarts*/ /*Key::from(&[0; 64])*/ Key::generate())
                    .cookie_content_security(CookieContentSecurity::Private)
                    .cookie_secure(true)
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(Duration::hours(1))
                            .session_ttl_extension_policy(TtlExtensionPolicy::OnEveryRequest),
                    )
                    .build(),
            )
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(()))
            .configure(api::config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
