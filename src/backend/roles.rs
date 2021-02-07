use anyhow::Result;

use crate::resources::role::Role;
use crate::resources::roles::Roles;
use crate::RmError;

pub(crate) async fn get_role(endpoint: &str) -> Result<Role, RmError> {
    let response = reqwest::get(endpoint).await?.text().await?;
    let result: Role = serde_json::from_str(&response)?;
    Ok(result)
}

pub(crate) async fn get_roles(endpoint: &str) -> Result<Roles, RmError> {
    let response = reqwest::get(endpoint).await?.text().await?;
    let result: Roles = serde_json::from_str(&response)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::common_field::CommonField;

    #[tokio::test]
    async fn test_get_role_by_id() {
        let endpoint = "https://www.redmine.org/roles/5.json";
        let result = get_role(endpoint).await.unwrap();
        let role = r#"
      {
          "id": 5,
          "name": "Release manager"
      }
    "#;
        let roles_object: CommonField = serde_json::from_str(role).unwrap();
        assert_eq!(result.role, roles_object);
    }

    #[tokio::test]
    async fn test_get_roles() {
        let endpoint = "https://www.redmine.org/roles.json";
        let result = get_roles(endpoint).await.unwrap();
        let roles = r#"[
        {
            "id": 3,
            "name": "Administrator"
        },
        {
            "id": 4,
            "name": "Contributor"
        },
        {
            "id": 5,
            "name": "Release manager"
        }
      ]"#;
        let roles_object: Vec<CommonField> = serde_json::from_str(roles).unwrap();
        assert_eq!(result.roles, roles_object);
    }
}
