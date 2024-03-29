use database::ConfigDatabase;
use jwt::ConfigJWT;
use modules::utility::env_utils::get_env_var;
use openapi::ConfigOpenApi;

use crate::constants;

mod database;
mod jwt;
mod openapi;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub fqdn: String,
    pub frontend: String,
    pub jwt: ConfigJWT,
    pub open_api: ConfigOpenApi,
    pub database: ConfigDatabase,
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}

impl Config {
    pub fn new() -> Self {
        let default_host: Option<&str> = Some(constants::CYB_HOST);
        let default_port: Option<&str> = Some(constants::CYB_PORT);
        let default_fqdn: Option<&str> = Some(constants::CYB_FQDN);
        let default_frontend: Option<&str> = Some(constants::CYB_FRONTEND);

        let host: String = get_env_var("CYB_HOST", default_host);
        let port: String = get_env_var("CYB_PORT", default_port);
        let fqdn: String = get_env_var("CYB_FQDN", default_fqdn);
        let frontend: String = get_env_var("CYB_FRONTEND", default_frontend);

        let jwt: ConfigJWT = ConfigJWT::new();
        let open_api: ConfigOpenApi = ConfigOpenApi::new();
        let database: ConfigDatabase = ConfigDatabase::new();

        Config {
            host,
            port,
            fqdn,
            frontend,
            jwt,
            open_api,
            database,
        }
    }

    pub fn url_api(&self) -> String {
        let host: String = self.host.clone();
        let port: String = self.port.clone();
        format!("{host}:{port}")
    }

    pub fn url_database(&self) -> String {
        self.database.url()
    }

    pub fn jwt_secret_bytes(&self) -> &[u8] {
        self.jwt.secret_bytes()
    }
}
