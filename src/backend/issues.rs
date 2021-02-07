use anyhow::Result;

use crate::resources::issue::Issue;
use crate::resources::issues::Issues;
use crate::RmError;

pub(crate) async fn get_issue(endpoint: &str) -> Result<Issue, RmError> {
    let response = reqwest::get(endpoint).await?.text().await?;
    let result: Issue = serde_json::from_str(&response)?;
    Ok(result)
}

pub(crate) async fn get_issues(endpoint: &str) -> Result<Issues, RmError> {
    let response = reqwest::get(endpoint).await?.text().await?;
    let result: Issues = serde_json::from_str(&response)?;
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
    }

    #[tokio::test]
    async fn test_get_issues() {
        let endpoint = "https://www.redmine.org/issues.json";
        let result = get_issues(endpoint).await.unwrap();
        assert!(result.issues.len() == 25);
    }

    //TODO: Add more tests
}
