use crate::resources::common_field::CommonField;
use chrono::{DateTime, Local};
use serde::Deserialize;

// Response model for the attachments API, which no command uses yet.
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Attachment {
    id: i32,
    filename: String,
    filesize: i32,
    content_type: String,
    description: String,
    content_url: String,
    thumbnail_url: String,
    author: CommonField,
    created_on: DateTime<Local>,
}
