use crate::config::Config;

pub(crate) fn build_endpoint_url(sub_command: &str, arg: &str) -> String {
    let baseurl = Config::get_env("BASE_URL".to_string()).unwrap();
    let endpoint = format!("{}/{}/{}.json", baseurl, sub_command, arg);
    endpoint
}
