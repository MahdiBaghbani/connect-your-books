use std::env;

pub fn get_env_var(env_var: &str) -> String {
    env::var(env_var).unwrap_or_else(|_| panic!("{env_var} is not set in .env file"))
}