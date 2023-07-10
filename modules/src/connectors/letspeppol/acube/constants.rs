use std::env;

use serde::{Deserialize, Serialize};

use crate::utility::env_utils::get_env_var;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    email: String,
    password: String,
}

#[derive(Debug)]
pub struct Constants {
    use_sandbox: bool,
    sandbox_base_url: String,
    sandbox_login_url: String,
    sandbox_user: String,
    sandbox_pass: String,
    production_base_url: String,
    production_login_url: String,
    production_user: String,
    production_pass: String,
    token_file: String,
}

impl Default for Constants {
    fn default() -> Self {
        Constants::new()
    }
}

impl Constants {
    pub fn new() -> Self {
        Constants {
            use_sandbox: get_env_var("CYB_ACUBE_SANDBOX") == "true",
            sandbox_base_url: get_env_var("CYB_ACUBE_SAND_URL"),
            sandbox_login_url: get_env_var("CYB_ACUBE_SAND_LOGIN_URL"),
            sandbox_user: get_env_var("CYB_ACUBE_SAND_USER"),
            sandbox_pass: get_env_var("CYB_ACUBE_SAND_PASS"),
            production_base_url: get_env_var("CYB_ACUBE_PROD_URL"),
            production_login_url: get_env_var("CYB_ACUBE_PROD_LOGIN_URL"),
            production_user: get_env_var("CYB_ACUBE_PROD_USER"),
            production_pass: get_env_var("CYB_ACUBE_PROD_PASS"),
            token_file: get_env_var("CYB_ACUBE_TOKEN_FILE"),
        }
    }

    pub fn base_url(&self) -> String {
        if self.use_sandbox {
            self.sandbox_base_url.clone()
        } else {
            self.production_base_url.clone()
        }
    }

    pub fn login_url(&self) -> String {
        if self.use_sandbox {
            self.sandbox_login_url.clone()
        } else {
            self.production_login_url.clone()
        }
    }

    pub fn login_user(&self) -> User {
        let user: String;
        let pass: String;

        if self.use_sandbox {
            user = self.sandbox_user.clone();
            pass = self.sandbox_pass.clone();
        } else {
            user = self.production_user.clone();
            pass = self.production_pass.clone();
        }

        User {
            email: user,
            password: pass,
        }
    }

    pub fn token_file(&self) -> String {
        self.token_file.clone()
    }
}
