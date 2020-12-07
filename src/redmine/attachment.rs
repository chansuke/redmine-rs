use crate::redmine::common_field::CommonField;
use serde::Deserialize;

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
    created_on: String,
}