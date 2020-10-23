use crate::blog::BlogPost;
use crate::db::DBState;

use actix_web::{post, web, HttpResponse};
use serde::Deserialize;

use std::env;

#[derive(Deserialize)]
pub struct AdminEdit {
    pub key: String,
    pub blog: BlogPost,
}

const KEY_ERROR: &str = "Could not find ADMIN_KEY variable, did you forget to set it?";

#[post("/api/admin/edit")]
pub async fn admin_edits(db: web::Data<DBState>, req: web::Json<AdminEdit>) -> HttpResponse {
    if req.key == env::var("ADMIN_KEY").expect(KEY_ERROR) {
        match db.upsert_blog(&req.blog).await {
            Ok(_) => HttpResponse::Ok().body(""),
            Err(e) => {
                HttpResponse::InternalServerError().body(format!("failed to set value: {}", e))
            }
        }
    } else {
        HttpResponse::Forbidden().body("incorrect key")
    }
}

#[derive(Deserialize)]
pub struct AdminDelete {
    pub key: String,
    pub url: String,
}

#[post("/api/admin/delete")]
pub async fn admin_delete(db: web::Data<DBState>, req: web::Json<AdminDelete>) -> HttpResponse {
    if req.key == env::var("ADMIN_KEY").expect(KEY_ERROR) {
        match db.delete_blog(&req.url).await {
            Ok(_) => HttpResponse::Ok().body(""),
            Err(e) => HttpResponse::NotFound().body(format!("error deleting: {}", e)),
        }
    } else {
        HttpResponse::Forbidden().body("incorrect key")
    }
}

#[post("/api/admin/key")]
pub async fn admin_key(req: String) -> HttpResponse {
    if req == env::var("ADMIN_KEY").expect(KEY_ERROR) {
        return HttpResponse::Ok().body("correct key");
    } else {
        return HttpResponse::Forbidden().body("incorrect key");
    }
}
