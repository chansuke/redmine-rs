use chrono::{DateTime, Local};
use serde::Deserialize;
#[derive(Deserialize, Debug, PartialEq)]
pub struct ProjectObject {
    pub id: i32,
    pub name: String,
    pub identifier: String,
    pub description: String,
    pub homepage: Option<String>,
    pub status: i32,
    pub created_on: DateTime<Local>,
    pub updated_on: DateTime<Local>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Project {
    pub project: ProjectObject,
}
