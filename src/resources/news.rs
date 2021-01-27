use crate::resources::common_field::CommonField;
use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct NewsObject {
    pub id: i32,
    pub project: CommonField,
    pub author: CommonField,
    pub title: String,
    pub summary: String,
    pub description: String,
    pub created_on: DateTime<Local>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct News {
    pub news: Vec<NewsObject>,
}
