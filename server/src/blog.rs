use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// Blog post data struct
pub struct BlogPost {
    pub title: String,
    pub contents: String,
    pub date: NaiveDate,
    pub url: String,
}

impl BlogPost {
    pub fn new(title: String, contents: String, date: NaiveDate, url: String) -> Self {
        Self {
            title,
            contents,
            date,
            url,
        }
    }
}
