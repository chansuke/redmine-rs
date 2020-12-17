use std::env;
use std::error;

#[derive(Debug, PartialEq)]
pub struct Config {}

impl Config {
    pub fn get_env(value: String) -> Result<String, Box<dyn error::Error>> {
        let env_var = env::var(value)?;
        Ok(env_var)
    }
}
