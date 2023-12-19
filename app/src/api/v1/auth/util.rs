use actix_identity::Identity;
use actix_session::Session;
use actix_web::{HttpMessage, HttpRequest};
use hsh::models::hash::Hash;
use itertools::Itertools;
use crate::db::models::User;

pub fn parse_username_from_email(address: &str) -> String {
    let name = &mut address.split('@').collect_vec()[0].chars();
    if address.ends_with("@deltalearns.ca") {
        name.next_back();
        name.next_back();
        name.next_back();
        name.next_back();
    }
    name.as_str().to_string()
}

pub fn generate_password_hash(password: &str, salt: &str) -> String {
    hex::encode(Hash::generate_hash(password, salt, "argon2i").unwrap())
}

pub fn login_user(req: &HttpRequest, user: &User, session: &Session) {
    Identity::login(&req.extensions(), user.email.to_string()).unwrap();
    if user.admin {
        session.insert("admin", true).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::api::v1::auth::util::parse_username_from_email;

    #[test]
    fn test_parse_username_from_email() {
        assert_eq!(parse_username_from_email("marvinf534@deltalearns.ca"), "marvin");
        assert_eq!(parse_username_from_email("test@example.com"), "test");
    }
}