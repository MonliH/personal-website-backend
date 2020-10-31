use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// Contact me submission
pub struct Submission {
    from: String,
    name: String,
    contents: String,
    date: NaiveDateTime,
}
