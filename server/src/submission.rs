use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
/// Contact me submission
pub struct Submission {
    from: String,
    name: String,
    contents: String,
    date: NaiveDateTime,
}

impl Submission {
    pub fn new(from: String, name: String, contents: String, date: NaiveDateTime) -> Self {
        Self {
            from,
            name,
            contents,
            date,
        }
    }
}
