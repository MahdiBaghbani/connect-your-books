use std::env;

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
        let host: String = env::var("CYB_HOST").expect("CYB_HOST is not set in .env file");
        let port: String = env::var("CYB_PORT").expect("CYB_PORT is not set in .env file");
        let fqdn: String = env::var("CYB_FQDN").expect("CYB_FQDN is not set in .env file");
        let frontend: String =
            env::var("CYB_FRONTEND").expect("CYB_FRONTEND is not set in .env file");
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
        let kind: String = env::var("CYB_DB_KIND").expect("CYB_DB_KIND is not set in .env file");
        let host: String = env::var("CYB_DB_HOST").expect("CYB_DB_HOST is not set in .env file");
        let port: String = env::var("CYB_DB_PORT").expect("CYB_DB_PORT is not set in .env file");
        let name: String = env::var("CYB_DB_NAME").expect("CYB_DB_NAME is not set in .env file");
        let user: String = env::var("CYB_DB_USER").expect("CYB_DB_USER is not set in .env file");
        let pass: String = env::var("CYB_DB_PASS").expect("CYB_DB_PASS is not set in .env file");

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
        let secret: String =
            env::var("CYB_JWT_SECRET").expect("CYB_JWT_SECRET is not set in .env file");
        let max_age: String =
            env::var("CYB_JWT_MAX_AGE").expect("CYB_JWT_MAX_AGE is not set in .env file");
        let expiration_time: String = env::var("CYB_JWT_EXPIRATION_TIME")
            .expect("CYB_JWT_EXPIRATION_TIME is not set in .env file");

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
