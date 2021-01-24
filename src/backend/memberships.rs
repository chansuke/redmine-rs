use crate::resources::memberships::Memberships;

pub(crate) async fn get_memberships(
    endpoint: &str,
) -> Result<Memberships, Box<dyn std::error::Error>> {
    let response = reqwest::get(endpoint).await?.text().await?;
    let result: Memberships = serde_json::from_str(&response)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_memberships() {
        let endpoint = "https://www.redmine.org/projects/redmine/memberships.json";
        let result = get_memberships(endpoint).await.unwrap();
        assert!(result.memberships.len() == 20);
    }
}
