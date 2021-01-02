use crate::resources::common_field::CommonField;
use crate::resources::custom_field::CustomField;
use serde::Deserialize;
use chrono::{DateTime, Local};

#[derive(Deserialize, Debug, PartialEq)]
pub struct IssueObject {
    pub id: i32,
    pub project: CommonField,
    pub tracker: CommonField,
    pub status: CommonField,
    pub priority: CommonField,
    pub author: CommonField,
    pub fixed_version: Option<CommonField>,
    pub subject: String,
    pub description: String,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub done_ratio: i32,
    pub estimated_hours: Option<i32>,
    pub custom_fields: Vec<CustomField>,
    pub created_on: DateTime<Local>,
    pub updated_on: DateTime<Local>,
    pub closed_on: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Issue {
    pub issue: IssueObject,
}
