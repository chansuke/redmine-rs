use anyhow::Result;

use crate::resources::user::User;
use crate::resources::users::Users;
use crate::RmError;

/// Get single user
pub(crate) async fn get_user(endpoint: &str) -> Result<User, RmError> {
    let response = reqwest::get(endpoint).await?.text().await?;
    let result: User = serde_json::from_str(&response)?;
    Ok(result)
}

/// Get all users
pub(crate) async fn get_users(endpoint: &str) -> Result<Users, RmError> {
    let response = reqwest::get(endpoint).await?.text().await?;
    let result: Users = serde_json::from_str(&response)?;
    Ok(result)
}

//TODO: write test
