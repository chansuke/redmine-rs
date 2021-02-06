use anyhow::Result;

use crate::resources::memberships::Memberships;
use crate::RmError;

pub(crate) async fn get_memberships(endpoint: &str) -> Result<Memberships, RmError> {
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
