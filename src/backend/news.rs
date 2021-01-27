use crate::resources::news::News;

pub(crate) async fn get_news(endpoint: &str) -> Result<News, Box<dyn std::error::Error>> {
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
