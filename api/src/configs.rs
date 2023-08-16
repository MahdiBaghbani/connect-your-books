use std::env;

use modules::utility::env_utils::get_env_var;

#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub fqdn: String,
    pub frontend: String,
    pub database: ConfigDatabase,
    pub jwt: ConfigJWT,
}

#[derive(Debug)]
pub struct ConfigDatabase {
    pub kind: String,
    pub host: String,
    pub port: String,
    pub name: String,
    pub user: String,
    pub pass: String,
}

#[derive(Debug)]
pub struct ConfigJWT {
    pub secret: String,
    pub max_age: i32,
    pub expiration_time: String,
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}

impl Config {
    pub fn new() -> Self {
        let host: String = get_env_var("CYB_HOST");
        let port: String = get_env_var("CYB_PORT");
        let fqdn: String = get_env_var("CYB_FQDN");
        let frontend: String = get_env_var("CYB_FRONTEND");
        let database: ConfigDatabase = ConfigDatabase::new();
        let jwt: ConfigJWT = ConfigJWT::new();

        Config {
            host,
            port,
            fqdn,
            frontend,
            database,
            jwt,
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

    pub fn secret_bytes(&self) -> &[u8] {
        self.jwt.secret_bytes()
    }
}

impl Default for ConfigDatabase {
    fn default() -> Self {
        ConfigDatabase::new()
    }
}
impl ConfigDatabase {
    pub fn new() -> Self {
        let kind: String = get_env_var("CYB_DB_KIND");
        let host: String = get_env_var("CYB_DB_HOST");
        let port: String = get_env_var("CYB_DB_PORT");
        let name: String = get_env_var("CYB_DB_NAME");
        let user: String = get_env_var("CYB_DB_USER");
        let pass: String = get_env_var("CYB_DB_PASS");

        ConfigDatabase {
            kind,
            host,
            port,
            name,
            user,
            pass,
        }
    }

    pub fn url(&self) -> String {
        let kind: String = self.kind.clone();
        let host: String = self.host.clone();
        let port: String = self.port.clone();
        let name: String = self.name.clone();
        let user: String = self.user.clone();
        let pass: String = self.pass.clone();

        format!("{kind}://{user}:{pass}@{host}:{port}/{name}")
    }
}

impl Default for ConfigJWT {
    fn default() -> Self {
        ConfigJWT::new()
    }
}

impl ConfigJWT {
    pub fn new() -> Self {
        let secret: String = get_env_var("CYB_JWT_SECRET");
        let max_age: String = get_env_var("CYB_JWT_MAX_AGE");
        let expiration_time: String = get_env_var("CYB_JWT_EXPIRATION_TIME");

        ConfigJWT {
            secret,
            max_age: max_age.parse::<i32>().unwrap_or(60),
            expiration_time,
        }
    }

    pub fn secret_bytes(&self) -> &[u8] {
        self.secret.as_bytes()
    }
}
