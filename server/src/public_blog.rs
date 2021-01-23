use actix_web::{get, web, HttpRequest, HttpResponse};

use crate::blog::{BlogPostDisplay, BlogPostNoHTML};
use crate::db::DBState;

#[get("/blog/entries/{starting}/{ending}")]
pub async fn blog_entries(
    db: web::Data<DBState>,
    web::Path((starting, ending)): web::Path<(usize, usize)>,
) -> HttpResponse {
    let len = match db.get_num_of_blogs().await {
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
        match &db.get_recent_blogs(starting as u32, ending as u32).await {
            Ok(blogs) => HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(blogs).expect("Failed to serialize blog posts")),
            Err(e) => HttpResponse::RangeNotSatisfiable().body(format!("no blogs found: {}", e)),
        }
    }
}

#[get("/blog/all_urls")]
pub async fn all_blog_urls(db: web::Data<DBState>) -> HttpResponse {
    let len = match db.get_num_of_blogs().await {
        Ok(n) => n,
        Err(_) => return HttpResponse::NotFound().body("length not found"),
    };

    // Edge case
    if len == 0 {
        return HttpResponse::Ok().body("[]");
    }

    match &db.get_all_urls().await {
        Ok(urls) => HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(urls).expect("Failed to serialize blog posts")),
        Err(e) => HttpResponse::InternalServerError().body(format!("failed to get query: {}", e)),
    }
}

#[get("/blog/pages")]
pub async fn number_of_blog_posts(db: web::Data<DBState>, _req: HttpRequest) -> HttpResponse {
    match db.get_num_of_blogs().await {
        Ok(nums) => HttpResponse::Ok()
            .content_type("text/plain")
            .body(nums.to_string()),
        Err(e) => HttpResponse::InternalServerError().body(format!("failed to get query: {}", e)),
    }
}

#[get("/blog/entry/display/{name}")]
pub async fn blog_post_display_by_name(
    db: web::Data<DBState>,
    web::Path(name): web::Path<String>,
) -> HttpResponse {
    match db
        .get_blog::<BlogPostDisplay>(&name, Some(&["html_preview", "md_contents"]))
        .await
    {
        Ok(post) => HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&post).expect("Failed to serialize blog")),
        Err(e) => HttpResponse::NotFound().body(format!("post not found: {}", e)),
    }
}

#[get("/blog/entry/admin/{name}")]
pub async fn blog_post_admin_by_name(
    db: web::Data<DBState>,
    web::Path(name): web::Path<String>,
) -> HttpResponse {
    match db
        .get_blog::<BlogPostNoHTML>(&name, Some(&["html_contents", "html_preview"]))
        .await
    {
        Ok(post) => HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&post).expect("Failed to serialize blog")),
        Err(e) => HttpResponse::NotFound().body(format!("post not found: {}", e)),
    }
}
