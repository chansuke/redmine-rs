use anyhow::Result;

use crate::resources::project::Project;
use crate::resources::projects::Projects;
use crate::RmError;

pub(crate) async fn get_project(endpoint: &str) -> Result<Project, RmError> {
    let response = reqwest::get(endpoint).await?.text().await?;
    let result: Project = serde_json::from_str(&response)?;
    Ok(result)
}

pub(crate) async fn get_projects(endpoint: &str) -> Result<Projects, RmError> {
    let response = reqwest::get(endpoint).await?.text().await?;
    let result: Projects = serde_json::from_str(&response)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_project() {
        let endpoint = "https://www.redmine.org/projects/1.json";
        let result = get_project(endpoint).await.unwrap();
        assert!(result.project.name == "Redmine");
        assert!(result.project.status == 1);
    }

    #[tokio::test]
    async fn test_get_projects() {
        let endpoint = "https://www.redmine.org/projects.json";
        let result = get_projects(endpoint).await.unwrap();
        assert!(result.total_count == 1);
    }
}
