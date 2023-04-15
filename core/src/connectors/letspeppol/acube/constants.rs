use std::env;

use serde::{Deserialize, Serialize};

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

impl Constants {
    // associated functions.
    pub fn create_constants() -> Constants {
        Constants {
            use_sandbox: env::var("CYB_ACUBE_SANDBOX")
                .expect("CYB_ACUBE_SANDBOX is not set in .env file")
                == "true",
            sandbox_base_url: env::var("CYB_ACUBE_SAND_URL")
                .expect("CYB_ACUBE_SAND_URL is not set in .env file"),
            sandbox_login_url: env::var("CYB_ACUBE_SAND_LOGIN_URL")
                .expect("CYB_ACUBE_SAND_LOGIN_URL is not set in .env file"),
            sandbox_user: env::var("CYB_ACUBE_SAND_USER")
                .expect("CYB_ACUBE_SAND_USER is not set in .env file"),
            sandbox_pass: env::var("CYB_ACUBE_SAND_PASS")
                .expect("CYB_ACUBE_SAND_PASS is not set in .env file"),
            production_base_url: env::var("CYB_ACUBE_PROD_URL")
                .expect("CYB_ACUBE_PROD_URL is not set in .env file"),
            production_login_url: env::var("CYB_ACUBE_PROD_LOGIN_URL")
                .expect("CYB_ACUBE_PROD_LOGIN_URL is not set in .env file"),
            production_user: env::var("CYB_ACUBE_PROD_USER")
                .expect("CYB_ACUBE_PROD_USER is not set in .env file"),
            production_pass: env::var("CYB_ACUBE_PROD_PASS")
                .expect("CYB_ACUBE_PROD_PASS is not set in .env file"),
            token_file: env::var("CYB_ACUBE_TOKEN_FILE")
                .expect("CYB_ACUBE_TOKEN_FILE is not set in .env file"),
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
