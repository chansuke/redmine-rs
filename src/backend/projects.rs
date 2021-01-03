use crate::resources::project::Project;

pub(crate) async fn get_project(endpoint: &str) -> Result<Project, Box<dyn std::error::Error>> {
    let response = reqwest::get(endpoint).await?.text().await?;
    let result: Project = serde_json::from_str(&response)?;
    println!("{:?}", result);
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
}
