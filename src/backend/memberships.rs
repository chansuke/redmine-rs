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
        // The member count changes over time, so compare it with the count the API reports.
        assert_eq!(result.memberships.len() as i32, result.total_count);
        let founder = &result.memberships[0];
        assert_eq!(founder.user.name, "Jean-Philippe Lang");
        assert_eq!(founder.roles[0].name, "Administrator");
    }
}
