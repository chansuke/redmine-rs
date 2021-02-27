use crate::resources::common_field::CommonField;
use crate::resources::custom_field::CustomField;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

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
    pub custom_fields: Option<Vec<CustomField>>,
    pub created_on: DateTime<Local>,
    pub updated_on: DateTime<Local>,
    pub closed_on: Option<DateTime<Local>>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Issue {
    pub issue: IssueObject,
}

/// project_id, subject is required
#[derive(Serialize, Debug, PartialEq)]
pub struct PostIssueObject<'a> {
    pub project_id: &'a str,
    pub subject: &'a str,
    pub description: Option<&'a str>,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct PostIssue<'a> {
    pub issue: PostIssueObject<'a>,
}
