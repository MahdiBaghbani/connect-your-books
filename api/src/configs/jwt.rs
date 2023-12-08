use modules::utility::env_utils::get_env_var;

use crate::constants;

#[derive(Debug, Clone)]
pub struct ConfigJWT {
    pub secret: String,
    pub max_age: i64,
    pub exp_time: String,
}

impl Default for ConfigJWT {
    fn default() -> Self {
        ConfigJWT::new()
    }
}

impl ConfigJWT {
    pub fn new() -> Self {
        let default_secret: Option<String> = Some(constants::CYB_JWT_SECRET.to_string());
        let default_max_age: Option<String> = Some(constants::CYB_JWT_MAX_AGE.to_string());
        let default_exp_time: Option<String> = Some(constants::CYB_JWT_EXP_TIME.to_string());


        let secret: String = get_env_var("CYB_JWT_SECRET", default_secret);
        let max_age: String = get_env_var("CYB_JWT_MAX_AGE", default_max_age);
        let exp_time: String = get_env_var("CYB_JWT_EXP_TIME", default_exp_time);

        ConfigJWT {
            secret,
            max_age: max_age.parse::<i64>().unwrap_or(60),
            exp_time,
        }
    }

    pub fn secret_bytes(&self) -> &[u8] {
        self.secret.as_bytes()
    }
}