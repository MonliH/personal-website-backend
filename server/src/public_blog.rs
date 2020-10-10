use actix_web::{get, web, HttpRequest, HttpResponse};

use crate::db::DBState;

#[get("/api/blog/entries/{starting}/{ending}")]
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
                    Err(_) => return HttpResponse::RangeNotSatisfiable().body("no blogs found"),
                },
            )
            .expect("Failed to serialize blog posts")
        })
}

#[get("/api/blog/pages")]
pub async fn blog_post_amounts(db: web::Data<DBState>, _req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(match db.get_num_of_blogs().await {
            Ok(nums) => nums.to_string(),
            Err(_) => {
                return HttpResponse::InternalServerError().body("failed to get query");
            }
        })
}

#[get("/api/blog/entry/{name}")]
pub async fn blog_post_by_name(
    db: web::Data<DBState>,
    web::Path(name): web::Path<String>,
) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(match db.get_blog(&name).await {
            Ok(post) => serde_json::to_string(&post).expect("Failed to serialize blog"),
            Err(_) => {
                return HttpResponse::NotFound().body("post not found");
            }
        })
}
