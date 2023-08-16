use serde::{Deserialize, Serialize};

use database::sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_connection: DatabaseConnection,
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
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
