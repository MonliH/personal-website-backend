use actix_web::{post, web, HttpResponse};

use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, offset};
use check_if_email_exists::{check_email, CheckEmailInput, Reachable};

use std::time::Duration;

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
            datetime: offset::Local::now().naive_local()
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
    datetime: NaiveDateTime
}

#[post("/contact/submit")]
pub async fn new_contact(
    db: web::Data<DBState>,
    submission: web::Json<Submission>,
) -> HttpResponse {
    let mut email_checker = CheckEmailInput::new(vec![submission.email.clone()]);

    let mut email_results_all = check_email(&email_checker).await;
    let email_results = email_results_all.pop().unwrap();

    if !email_results.syntax.is_valid_syntax {
        return HttpResponse::Forbidden().body("invalid email format provided");
    }

    if let Ok(misc) = email_results.misc {
        if misc.is_disposable {
            return HttpResponse::Forbidden().body("disposable emails are not accepted");
        }
    }

    match email_results.is_reachable {
        Reachable::Risky | Reachable::Safe => {}
        _ => {
            return HttpResponse::Forbidden().body("the email doesn't exist");
        }
    }

    match db.insert_submission(submission.into_inner().with_date()).await {
        Ok(_) => HttpResponse::Ok().body(""),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("failed to set value: {}", e))
        }
    }
}
