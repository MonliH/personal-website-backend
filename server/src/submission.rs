use actix_web::{get, post, web, HttpResponse};
use actix_web_httpauth::extractors::basic::BasicAuth;

use chrono::{offset, NaiveDateTime};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::admin::{bad_key, validate_key};
use crate::db::DBState;

#[derive(Debug, Serialize, Deserialize)]
/// Contact me submission
pub struct Submission {
    email: String,
    name: String,
    subject: String,
    contents: String,
}

impl Submission {
    fn with_date(self) -> SubmissionDate {
        return SubmissionDate {
            email: self.email,
            name: self.name,
            subject: self.subject,
            contents: self.contents,
            datetime: offset::Local::now().naive_local(),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// Contact me submission with date
pub struct SubmissionDate {
    email: String,
    name: String,
    subject: String,
    contents: String,
    datetime: NaiveDateTime,
}

fn is_email(text: &str) -> bool {
    lazy_static! {
        static ref EMAIL_REGEX: Regex =
            Regex::new(
                r"^[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?$"
            ).unwrap();
    }
    EMAIL_REGEX.is_match(text)
}

#[post("/contact/submit")]
pub async fn new_contact(
    db: web::Data<DBState>,
    submission: web::Json<Submission>,
) -> HttpResponse {
    if !is_email(&submission.email) {
        return HttpResponse::Forbidden().body("Invalid email");
    }

    match db
        .insert_submission(submission.into_inner().with_date())
        .await
    {
        Ok(_) => HttpResponse::Ok().body("Submission sent!"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to submit submission"),
    }
}

#[get("/admin/contacts")]
pub async fn number_of_contacts(db: web::Data<DBState>, auth: BasicAuth) -> HttpResponse {
    match auth.password() {
        Some(pwd) if validate_key(auth.user_id(), pwd) => match db.get_num_of_contacts().await {
            Ok(num) => HttpResponse::Ok().body(num.to_string()),
            Err(e) => HttpResponse::InternalServerError()
                .body(format!("failed to get number of contacts: {}", e)),
        },
        _ => bad_key(),
    }
}

#[get("/admin/contact/range/{start}/{end}")]
pub async fn contact_range(
    db: web::Data<DBState>,
    web::Path((starting, ending)): web::Path<(usize, usize)>,
    auth: BasicAuth,
) -> HttpResponse {
    let len = match db.get_num_of_contacts().await {
        Ok(n) => n,
        Err(_) => return HttpResponse::NotFound().body("length not found"),
    };

    // Edge case
    if starting == 0 && ending == 0 {
        return HttpResponse::Ok().body("[]");
    }

    if ending > len || starting > len {
        // Out of range
        return HttpResponse::RangeNotSatisfiable().body("specified range out of range");
    } else {
        match auth.password() {
            Some(pwd) if validate_key(auth.user_id(), pwd) => match &db
                .get_recent_submissions(starting as u32, ending as u32)
                .await
            {
                Ok(submissions) => {
                    return HttpResponse::Ok().content_type("application/json").body(
                        serde_json::to_string(submissions)
                            .expect("Failed to serialize submissions"),
                    )
                }
                Err(e) => {
                    HttpResponse::RangeNotSatisfiable().body(format!("no submissions found: {}", e))
                }
            },
            _ => bad_key(),
        }
    }
}
