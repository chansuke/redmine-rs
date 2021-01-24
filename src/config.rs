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
    fn get_baseurl_env() {
        env::set_var("BASE_URL", "https://test.redmine.org");
        assert!(Config::get_env("BASE_URL".to_string()).unwrap() == "https://test.redmine.org");
    }

    #[test]
    fn get_project_env() {
        env::set_var("PROJECT", "redmine");
        assert!(Config::get_env("PROJECT".to_string()).unwrap() == "redmine");
    }

    #[test]
    fn empty_env_variable() {
        env::set_var("BASE_URL", "");
        let env_var = Config::get_env("BASE_URL".to_string()).unwrap();
        assert!(env_var.is_empty());
    }
}
