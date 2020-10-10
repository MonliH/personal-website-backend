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

impl BlogPost {
    pub fn new(
        title: String,
        html_contents: String,
        md_contents: String,
        date: NaiveDate,
        url: String,
    ) -> Self {
        Self {
            title,
            html_contents,
            md_contents,
            date,
            url,
        }
    }
}
