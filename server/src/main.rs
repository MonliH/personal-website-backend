mod blog;
mod db;

use db::DB;

use actix_files::{Files, NamedFile};
use actix_web::{get, middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer};

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use dotenv::dotenv;
use env_logger::Env;

use std::io;
use std::sync::Arc;

type DBState = Arc<DB>;

async fn home(_req: HttpRequest) -> io::Result<NamedFile> {
    Ok(NamedFile::open("www/build/index.html")?)
}

#[get("/api/blog/entries/{starting}/{ending}")]
async fn blog_entries(
    db: web::Data<DBState>,
    web::Path((starting, ending)): web::Path<(usize, usize)>,
) -> HttpResponse {
    let len = match db.get_num_of_blogs().await {
        Ok(n) => n,
        Err(_) => return HttpResponse::NotFound().body("length not found"),
    };
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
async fn blog_post_amounts(db: web::Data<DBState>, _req: HttpRequest) -> HttpResponse {
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
async fn blog_post_by_name(
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    dotenv().ok();

    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let db = Arc::new(DB::new().await.expect("Failed to get DB handle"));

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .data(Arc::clone(&db) as DBState)
            .service(blog_post_by_name)
            .service(blog_post_amounts)
            .service(blog_entries)
            .service(
                Files::new("/*", "www/build/")
                    .index_file("www/build/index.html")
                    .default_handler(web::route().to(home)),
            )
            .wrap(Logger::default())
    })
    .bind_openssl("127.0.0.1:8080", builder)?
    .run();

    println!("Server running");

    server.await
}
