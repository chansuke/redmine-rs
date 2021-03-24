use anyhow::Result;

use crate::config::Config;
use crate::RmError;

pub(crate) fn build_endpoint(sub_command: &str, arg: &str) -> Result<String, RmError> {
    let baseurl = Config::get_env("REDMINE_BASE_URL".to_string())?;
    let project = Config::get_env("REDMINE_PROJECT".to_string())?;
    let endpoint;
    if arg.is_empty() && sub_command != "memberships" {
        endpoint = format!("{}/{}.json", baseurl, sub_command);
    } else if sub_command == "memberships" {
        endpoint = format!("{}/projects/{}/memberships.json", baseurl, project);
    } else {
        endpoint = format!("{}/{}/{}.json", baseurl, sub_command, arg);
    }
    Ok(endpoint)
}

pub(crate) fn append_apikey_clause(endpoint: &str) -> Result<String, RmError> {
    let apikey = Config::get_env("REDMINE_API_KEY".to_string())?;
    let endpoint = format!("{}?key={}", endpoint, apikey);
    Ok(endpoint)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_build_endpoint() {
        env::set_var("REDMINE_BASE_URL", "https://test.redmine.org");
        assert!(
            build_endpoint("issues", "199").unwrap() == "https://test.redmine.org/issues/199.json"
        );
    }
}
