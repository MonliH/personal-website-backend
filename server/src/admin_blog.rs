use crate::admin::protected;
use crate::blog::BlogPostNoHTML;
use crate::db::DBState;

use actix_web::{delete, get, put, web, HttpResponse};
use actix_web_httpauth::extractors::basic::BasicAuth;

use std::sync::Arc;

#[put("/admin/edit")]
pub async fn admin_edits(
    db: web::Data<DBState>,
    blog: web::Json<BlogPostNoHTML>,
    auth: BasicAuth,
) -> HttpResponse {
    protected(auth.user_id(), auth.password(), || async {
        match Arc::clone(db.get_ref()).upsert_blog(blog.render()).await {
            Ok(_) => HttpResponse::Ok().body(""),
            Err(e) => {
                HttpResponse::InternalServerError().body(format!("failed to set value: {}", e))
            }
        }
    })
    .await
}

#[delete("/admin/delete/{url}")]
pub async fn admin_delete(
    db: web::Data<DBState>,
    web::Path(url): web::Path<String>,
    auth: BasicAuth,
) -> HttpResponse {
    protected(auth.user_id(), auth.password(), || async {
        match db.delete_blog(&url).await {
            Ok(_) => HttpResponse::Ok().body(""),
            Err(e) => HttpResponse::NotFound().body(format!("error deleting: {}", e)),
        }
    })
    .await
}

#[get("/admin/key")]
pub async fn admin_key(auth: BasicAuth) -> HttpResponse {
    protected(auth.user_id(), auth.password(), || async {
        HttpResponse::Ok().body("correct key")
    })
    .await
}
