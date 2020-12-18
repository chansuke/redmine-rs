use crate::config::Config;

pub(crate) fn build_endpoint(
    sub_command: &str,
    arg: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let baseurl = Config::get_env("BASE_URL".to_string())?;
    let endpoint = format!("{}/{}/{}.json", baseurl, sub_command, arg);
    Ok(endpoint)
}
