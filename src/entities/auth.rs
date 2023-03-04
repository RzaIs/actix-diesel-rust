use serde::{Deserialize, Serialize};
use crate::entities::user::User;

#[derive(Deserialize)]
pub struct RegisterDTO {
    pub email: String,
    pub username: String,
    pub secret: String
}

#[derive(Deserialize)]
pub struct LoginDTO {
    pub username: String,
    pub secret: String
}

#[derive(Serialize)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub user: User,
    pub tokens: Tokens
}

// JWT

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub exp: usize
}

#[derive(Serialize, Deserialize)]
pub struct AuthToken {
    pub id: i32,
}
