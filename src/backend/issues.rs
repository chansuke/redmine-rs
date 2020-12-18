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
   match command {
        "get" => reqwest::get(endpoint).await?.text().await,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_api() {
        let endpoint = "https://www.redmine.org/issues/1.json";
        let command = "get";
        let result = call_api(endpoint, command).await.unwrap();
        assert!(result.issue.id == 1);
        assert!(result.issue.custom_fields.len() == 2);
    }
}
