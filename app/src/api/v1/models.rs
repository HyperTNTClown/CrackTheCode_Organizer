use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserLoginRequest {
    pub(crate) email: String,
    pub(crate) password: String
}

#[derive(Deserialize, Serialize)]
pub struct UserRegisterRequest {
    pub(crate) password: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct IsAdminRequest {
    pub(crate) email: String,
}

#[derive(Serialize)]
pub struct IsAdminResponse {
    pub(crate) is_admin: bool,
}