use std::env;

use anyhow::Result;

use crate::RmError;

#[derive(Debug, PartialEq)]
pub struct Config {}

impl Config {
    pub fn get_env(value: String) -> Result<String, RmError> {
        let env_var = env::var(value)?;
        Ok(env_var)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Each test uses its own variable, because tests run in parallel in one process.
    #[test]
    fn get_baseurl_env() {
        env::set_var("REDMINE_RS_TEST_SET_URL", "https://test.redmine.org");
        assert!(
            Config::get_env("REDMINE_RS_TEST_SET_URL".to_string()).unwrap()
                == "https://test.redmine.org"
        );
    }

    #[test]
    fn get_project_env() {
        env::set_var("REDMINE_RS_TEST_SET_PROJECT", "redmine");
        assert!(Config::get_env("REDMINE_RS_TEST_SET_PROJECT".to_string()).unwrap() == "redmine");
    }

    #[test]
    fn empty_env_variable() {
        env::set_var("REDMINE_RS_TEST_EMPTY", "");
        let env_var = Config::get_env("REDMINE_RS_TEST_EMPTY".to_string()).unwrap();
        assert!(env_var.is_empty());
    }
}
