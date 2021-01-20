use crate::blog::BlogPostNoHTML;
use crate::db::DBState;

use actix_web::{get, delete, put, web, HttpResponse};
use actix_web_httpauth::extractors::basic::BasicAuth;

use std::env;
use std::borrow::Cow;

pub fn validate_key(username: &Cow<'static, str>, pwd: &Cow<'static, str>) -> bool {
    username == &env::var("ADMIN_USERNAME").expect(KEY_ERROR) && pwd == &env::var("ADMIN_KEY").expect(KEY_ERROR)
}

const KEY_ERROR: &str = "Could not find ADMIN_KEY variable, did you forget to set it?";

#[put("/admin/edit")]
pub async fn admin_edits(db: web::Data<DBState>, blog: web::Json<BlogPostNoHTML>, auth: BasicAuth) -> HttpResponse {
    match auth.password() {
        Some(pwd) if validate_key(auth.user_id(), pwd) => match db.upsert_blog(&(blog.into_inner().render())).await {
            Ok(_) => HttpResponse::Ok().body(""),
            Err(e) => {
                HttpResponse::InternalServerError().body(format!("failed to set value: {}", e))
            }
        }
        _ => {
            HttpResponse::Forbidden().body("incorrect key")
        }
    }
}

#[delete("/admin/delete/{url}")]
pub async fn admin_delete(db: web::Data<DBState>, web::Path(url): web::Path<String>, auth: BasicAuth) -> HttpResponse {
    match auth.password() {
        Some(pwd) if validate_key(auth.user_id(), pwd) => {
            match db.delete_blog(&url).await {
                Ok(_) => HttpResponse::Ok().body(""),
                Err(e) => HttpResponse::NotFound().body(format!("error deleting: {}", e)),
            }
        }
        _ => {
            HttpResponse::Forbidden().body("incorrect key")
        }
    }
}

#[get("/admin/key")]
pub async fn admin_key(auth: BasicAuth) -> HttpResponse {
    match auth.password() {
        Some(pwd) if validate_key(auth.user_id(), pwd) => {
            HttpResponse::Ok().body("correct key")
        }
        _ => {
            HttpResponse::Forbidden().body("incorrect key")
        }
    }
}
