use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// Blog post data struct
pub struct BlogPost {
    pub title: String,
    pub html_contents: String,
    pub md_contents: String,
    pub date: NaiveDate,
    pub url: String,
}
