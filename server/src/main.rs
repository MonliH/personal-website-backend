mod blog;
mod db;

use db::DB;

use actix_files::{Files, NamedFile};
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer};

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use std::io;
use std::sync::Arc;

use dotenv::dotenv;

#[derive(Clone)]
struct DBState(Arc<DB>);

async fn home(_req: HttpRequest) -> io::Result<NamedFile> {
    Ok(NamedFile::open("www/build/index.html")?)
}

#[get("/api/blog/entries/{starting}/{ending}")]
async fn blog_entries(db: web::Data<DBState>, web::Path((starting, ending)): web::Path<(usize, usize)>) -> HttpResponse {
    let len = match db.0.get_blog_nums().await {
        Ok(n) => n,
        Err(_) => return HttpResponse::NotFound().body("length not found"),
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .body(if ending > len || starting > len {
            // Out of range
            return HttpResponse::RangeNotSatisfiable().body("specified range out of range");
        } else {
            serde_json::to_string(&db.0.get_recent_blogs(starting as u32, ending as u32).await.unwrap()[starting..ending]).expect("Failed to serialize blog posts")
        })
}

#[get("/api/blog/pages")]
async fn blog_post_amounts(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("10")
}

#[get("/api/blog/entry/{name}")]
async fn blog_post_by_name(db: web::Data<DBState>, web::Path(name): web::Path<String>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(match db.0.get_blog(&name).await {
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

    let db = Arc::new(DB::new().await.expect("Failed to get DB handle"));

    println!("{:?}", db.get_recent_blogs(0, 10).await);

    // for i in 1..10 {
    //     db.upsert_blog(&BlogPost::new(
    //         "Setting up a wasm".to_string(),
    //         "<p>hi</p>".to_string(),
    //         NaiveDate::from_ymd(2020, 09, i),
    //         format!("wasm-react-ts-{}", i),
    //     ))
    //     .await
    //     .expect("failed to insert blog");
    // }

    // println!("{:?}", db.get_blog("wasm-react-ts").await);

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(DBState(Arc::clone(&db)))
            .service(blog_post_by_name)
            .service(blog_post_amounts)
            .service(blog_entries)
            .service(
                Files::new("/*", "www/build/")
                    .index_file("www/build/index.html")
                    .default_handler(web::route().to(home)),
            )
    })
    .bind_openssl("127.0.0.1:8080", builder)?
    .run();

    println!("Server running");

    server.await
}
