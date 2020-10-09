use crate::blog::BlogPost;

use actix_web::{post, web, HttpResponse};
use serde::Deserialize;

use std::env;

#[derive(Deserialize)]
pub struct AdminEdit {
    key: String,
    blog: BlogPost,
}

#[post("/api/admin/edit")]
pub async fn admin_edits(
    req: web::Json<AdminEdit>
) -> HttpResponse {
    HttpResponse::Ok().body("hi")
}

#[post("/api/admin/key")]
pub async fn admin_key(req: String) -> HttpResponse {
    if req == env::var("ADMIN_KEY").expect("Could not find ADMIN_KEY variable, did you forget to set it?") {
        return HttpResponse::Ok().body("correct key")
    } else {
        return HttpResponse::Forbidden().body("incorrect key")
    }
}
