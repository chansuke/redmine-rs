use crate::resources::issue::Issue;

pub(crate) async fn call_api(
    endpoint: &str,
    command: &str,
) -> Result<Issue, Box<dyn std::error::Error>> {
    let response = handle_request(endpoint, command).await.unwrap();
    let result: Issue = serde_json::from_str(&response)?;
    Ok(result)
}

async fn handle_request(endpoint: &str, command: &str) -> Result<String, reqwest::Error> {
    let response = match command {
        "get" => reqwest::get(endpoint).await?.text().await,
        _ => unreachable!(),
    };
    response
}
