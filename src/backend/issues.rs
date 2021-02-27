use anyhow::Result;
use reqwest::Client;

use crate::resources::issues::Issues;
use crate::RmError;
use crate::{
    config::Config,
    resources::issue::{Issue, PostIssue, PostIssueObject},
};

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

pub(crate) async fn post_issue(endpoint: &str, args: Vec<&str>) -> Result<Issue, RmError> {
    let project = Config::get_env("PROJECT".to_string())?;

    let issue_object = PostIssueObject {
        project_id: &project,
        subject: args[0],
        description: Some(args[1]),
    };
    let api_key = Config::get_env("API_KEY".to_string())?;
    let issue = PostIssue {
        issue: issue_object,
    };

    let response = Client::new()
        .post(endpoint)
        .header("X-Redmine-API-Key", api_key)
        .json(&issue)
        .send()
        .await?;

    let result: Issue = response.json().await?;

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
}
