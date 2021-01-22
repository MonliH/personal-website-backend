use crate::admin::{bad_key, validate_key};
use crate::blog::BlogPostNoHTML;
use crate::db::DBState;

use actix_web::{delete, get, put, web, HttpResponse};
use actix_web_httpauth::extractors::basic::BasicAuth;

#[put("/admin/edit")]
pub async fn admin_edits(
    db: web::Data<DBState>,
    blog: web::Json<BlogPostNoHTML>,
    auth: BasicAuth,
) -> HttpResponse {
    match auth.password() {
        Some(pwd) if validate_key(auth.user_id(), pwd) => {
            match db.upsert_blog(&(blog.into_inner().render())).await {
                Ok(_) => HttpResponse::Ok().body(""),
                Err(e) => {
                    HttpResponse::InternalServerError().body(format!("failed to set value: {}", e))
                }
            }
        }
        _ => bad_key(),
    }
}

#[delete("/admin/delete/{url}")]
pub async fn admin_delete(
    db: web::Data<DBState>,
    web::Path(url): web::Path<String>,
    auth: BasicAuth,
) -> HttpResponse {
    match auth.password() {
        Some(pwd) if validate_key(auth.user_id(), pwd) => match db.delete_blog(&url).await {
            Ok(_) => HttpResponse::Ok().body(""),
            Err(e) => HttpResponse::NotFound().body(format!("error deleting: {}", e)),
        },
        _ => bad_key(),
    }
}

#[get("/admin/key")]
pub async fn admin_key(auth: BasicAuth) -> HttpResponse {
    match auth.password() {
        Some(pwd) if validate_key(auth.user_id(), pwd) => HttpResponse::Ok().body("correct key"),
        _ => bad_key(),
    }
}
