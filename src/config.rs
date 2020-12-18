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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_env_variable() {
        env::set_var("BASE_URL", "https://test.redmine.org");
        assert!(Config::get_env("BASE_URL".to_string()).unwrap() == "https://test.redmine.org");
    }

    #[test]
    #[should_panic(expected = "Invalid argument (os error 22)")]
    fn empty_env_variable() {
        env::set_var("", "");
        Config::get_env("BASE_URL".to_string()).unwrap();
    }
}
