use anyhow::Result;

use crate::resources::news::News;
use crate::RmError;

pub(crate) async fn get_news(endpoint: &str) -> Result<News, RmError> {
    let response = reqwest::get(endpoint).await?.text().await?;
    let result: News = serde_json::from_str(&response)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_news() {
        let endpoint = "https://www.redmine.org/news.json";
        let result = get_news(endpoint).await.unwrap();
        assert!(result.news.len() == 25);
    }
}
