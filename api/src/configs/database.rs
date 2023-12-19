use modules::utility::env_utils::get_env_var;

use crate::constants;

#[derive(Debug, Clone)]
pub struct ConfigDatabase {
    pub kind: String,
    pub host: String,
    pub port: String,
    pub name: String,
    pub user: String,
    pub pass: String,
}

impl Default for ConfigDatabase {
    fn default() -> Self {
        ConfigDatabase::new()
    }
}

impl ConfigDatabase {
    pub fn new() -> Self {
        let default_kind: Option<&str> = Some(constants::CYB_DB_KIND);
        let default_host: Option<&str> = Some(constants::CYB_DB_HOST);
        let default_port: Option<&str> = Some(constants::CYB_DB_PORT);
        let default_name: Option<&str> = Some(constants::CYB_DB_NAME);
        let default_user: Option<&str> = Some(constants::CYB_DB_USER);
        let default_pass: Option<&str> = Some(constants::CYB_DB_PASS);

        let kind: String = get_env_var("CYB_DB_KIND", default_kind);
        let host: String = get_env_var("CYB_DB_HOST", default_host);
        let port: String = get_env_var("CYB_DB_PORT", default_port);
        let name: String = get_env_var("CYB_DB_NAME", default_name);
        let user: String = get_env_var("CYB_DB_USER", default_user);
        let pass: String = get_env_var("CYB_DB_PASS", default_pass);

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
