use std::env;

pub fn get_env_var(env_var: &str, default_value: Option<String>) -> String {
    match default_value {
        Some(default_value) => env::var(env_var).unwrap_or(default_value),
        None => env::var(env_var).unwrap_or_else(|_| panic!("{env_var} is not set in .env file"))
    }
}
