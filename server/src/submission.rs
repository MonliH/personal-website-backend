use actix_web::{delete, get, post, web, HttpResponse};
use actix_web_httpauth::extractors::basic::BasicAuth;

use chrono::{offset, NaiveDateTime};
use mongodb::bson::oid::ObjectId;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::admin::protected;
use crate::db::DBState;

#[derive(Debug, Serialize, Deserialize)]
/// Contact me submission
pub struct Submission {
    email: String,
    sender_name: String,
    contents: String,
}

impl Submission {
    fn with_date(self) -> SubmissionDate {
        return SubmissionDate {
            email: self.email,
            sender_name: self.sender_name,
            contents: self.contents,
            datetime: offset::Local::now().naive_local(),
            id: None,
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// Contact me submission with date
pub struct SubmissionDate {
    email: String,
    sender_name: String,
    contents: String,
    datetime: NaiveDateTime,
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
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

    match db.insert_contact(submission.into_inner().with_date()).await {
        Ok(_) => HttpResponse::Ok().body("Submission sent!"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to submit submission"),
    }
}

#[get("/admin/contacts")]
pub async fn number_of_contacts(db: web::Data<DBState>, auth: BasicAuth) -> HttpResponse {
    protected(auth.user_id(), auth.password(), || async {
        match db.get_num_of_contacts().await {
            Ok(num) => HttpResponse::Ok().body(num.to_string()),
            Err(e) => HttpResponse::InternalServerError()
                .body(format!("failed to get number of contacts: {}", e)),
        }
    })
    .await
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
        HttpResponse::Ok().body("[]")
    } else if ending > len || starting > len {
        // Out of range
        HttpResponse::RangeNotSatisfiable().body("specified range out of range")
    } else {
        protected(auth.user_id(), auth.password(), || async {
            match &db
                .get_recent_submissions(starting as u32, ending as u32)
                .await
            {
                Ok(submissions) => HttpResponse::Ok().content_type("application/json").body(
                    serde_json::to_string(submissions).expect("Failed to serialize submissions"),
                ),
                Err(e) => {
                    HttpResponse::RangeNotSatisfiable().body(format!("no submissions found: {}", e))
                }
            }
        })
        .await
    }
}

#[delete("/admin/contact/delete/{contact_id}")]
pub async fn delete_contact(
    db: web::Data<DBState>,
    web::Path(contact_id): web::Path<String>,
    auth: BasicAuth,
) -> HttpResponse {
    protected(auth.user_id(), auth.password(), || async {
        match &db.delete_contact(&contact_id).await {
            Ok(_) => HttpResponse::Ok().body(""),
            Err(_) => HttpResponse::InternalServerError().body("failed to delete contact"),
        }
    })
    .await
}
