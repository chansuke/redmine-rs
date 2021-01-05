use crate::resources::user::User;

pub(crate) async fn get_user(endpoint: &str) -> Result<User, Box<dyn std::error::Error>> {
    let response = reqwest::get(endpoint).await?.text().await?;
    let result: User = serde_json::from_str(&response)?;
    println!("{:?}", result);
    Ok(result)
}
