use crate::resources::issue::Issue;
use crate::resources::issues::Issues;

pub(crate) async fn get_issue(endpoint: &str) -> Result<Issue, Box<dyn std::error::Error>> {
    let response = reqwest::get(endpoint).await?.text().await?;
    let result: Issue = serde_json::from_str(&response)?;
    println!("{:?}", result);
    Ok(result)
}

pub(crate) async fn get_issues(endpoint: &str) -> Result<Issues, Box<dyn std::error::Error>> {
    let response = reqwest::get(endpoint).await?.text().await?;
    let result: Issues = serde_json::from_str(&response)?;
    println!("{:?}", result);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_issue() {
        let endpoint = "https://www.redmine.org/issues/1.json";
        let result = get_issue(endpoint).await.unwrap();
        assert!(result.issue.id == 1);
        assert!(result.issue.custom_fields.len() == 2);
    }
}
