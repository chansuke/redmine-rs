use crate::config::Config;

pub(crate) fn build_endpoint(
    sub_command: &str,
    arg: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let baseurl = Config::get_env("BASE_URL".to_string())?;
    let endpoint;
    if arg.is_empty() {
        endpoint = format!("{}/{}.json", baseurl, sub_command);
    } else {
        endpoint = format!("{}/{}/{}.json", baseurl, sub_command, arg);
    }
    Ok(endpoint)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_build_endpoint() {
        env::set_var("BASE_URL", "https://test.redmine.org");
        assert!(
            build_endpoint("issues", "199").unwrap() == "https://test.redmine.org/issues/199.json"
        );
    }
}
