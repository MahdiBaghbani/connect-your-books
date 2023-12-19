use serde::{Deserialize, Serialize};

use database::sea_orm::DatabaseConnection;

use crate::configs::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_connection: DatabaseConnection,
    pub configs: Config,
}

#[derive(Debug, Deserialize)]
pub struct SignUpUserSchema {
    pub name_first: String,
    pub name_last: String,
    pub name_user: String,
    pub password: String,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SignInUserSchema {
    pub name_user: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    // (subject): Subject of the JWT (the user)
    pub sub: String,
    // (issued at time): Time at which the JWT was issued;
    // can be used to determine age of the JWT.
    pub iat: i64,
    // (expiration time): Time after which the JWT expires
    pub exp: i64,
}
