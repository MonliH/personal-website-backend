use actix_web::{get, web, HttpRequest, HttpResponse};

use crate::db::DBState;
use crate::blog::{BlogPostNoHTML, BlogPostDisplay};

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
        return HttpResponse::Ok().body("[]");
    }

    HttpResponse::Ok()
        .content_type("application/json")
        .body(if ending > len || starting > len {
            // Out of range
            return HttpResponse::RangeNotSatisfiable().body("specified range out of range");
        } else {
            serde_json::to_string(
                match &db.get_recent_blogs(starting as u32, ending as u32).await {
                    Ok(blogs) => blogs,
                    Err(e) => {
                        return HttpResponse::RangeNotSatisfiable()
                            .body(format!("no blogs found: {}", e))
                    }
                },
            )
            .expect("Failed to serialize blog posts")
        })
}

#[get("/blog/all_urls")]
pub async fn all_blog_urls(
    db: web::Data<DBState>,
) -> HttpResponse {
    let len = match db.get_num_of_blogs().await {
        Ok(n) => n,
        Err(_) => return HttpResponse::NotFound().body("length not found"),
    };

    // Edge case
    if len == 0 {
        return HttpResponse::Ok().body("[]");
    }

    HttpResponse::Ok()
        .content_type("application/json")
        .body(
            serde_json::to_string(
                match &db.get_all_urls().await {
                    Ok(urls) => urls,
                    Err(e) => {
                        return HttpResponse::InternalServerError()
                            .body(format!("failed to get query: {}", e))
                    }
                },
            )
            .expect("Failed to serialize blog posts")
        )
}

#[get("/blog/pages")]
pub async fn blog_post_amounts(db: web::Data<DBState>, _req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(match db.get_num_of_blogs().await {
            Ok(nums) => nums.to_string(),
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("failed to get query: {}", e));
            }
        })
}

#[get("/blog/entry/display/{name}")]
pub async fn blog_post_display_by_name(
    db: web::Data<DBState>,
    web::Path(name): web::Path<String>,
) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(match db.get_blog::<BlogPostDisplay>(&name, Some(&["html_preview", "md_contents"])).await {
            Ok(post) => serde_json::to_string(&post).expect("Failed to serialize blog"),
            Err(e) => {
                return HttpResponse::NotFound().body(format!("post not found: {}", e));
            }
        })
}

#[get("/blog/entry/admin/{name}")]
pub async fn blog_post_admin_by_name(
    db: web::Data<DBState>,
    web::Path(name): web::Path<String>,
) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(match db.get_blog::<BlogPostNoHTML>(&name, Some(&["html_contents", "html_preview"])).await {
            Ok(post) => serde_json::to_string(&post).expect("Failed to serialize blog"),
            Err(e) => {
                return HttpResponse::NotFound().body(format!("post not found: {}", e));
            }
        })
}
