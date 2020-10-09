mod admin;
mod blog;
mod db;
mod public_blog;

use db::{DBState, DB};

use actix_files::{Files, NamedFile};
use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::{middleware::Logger, web, App, HttpRequest, HttpServer};

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use dotenv::dotenv;
use env_logger::Env;

use std::io;
use std::sync::Arc;
use std::time::Duration;

async fn home(_req: HttpRequest) -> io::Result<NamedFile> {
    Ok(NamedFile::open("www/build/index.html")?)
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

    let store = MemoryStore::new();

    let server = HttpServer::new(move || {
        App::new()
            .data(Arc::clone(&db) as DBState)
            .service(admin::admin_edits)
            .service(admin::admin_key)
            .service(public_blog::blog_post_by_name)
            .service(public_blog::blog_post_amounts)
            .service(public_blog::blog_entries)
            .service(
                Files::new("/*", "www/build/")
                    .index_file("www/build/index.html")
                    .default_handler(web::route().to(home)),
            )
            .wrap(
                RateLimiter::new(MemoryStoreActor::from(store.clone()).start())
                    .with_interval(Duration::from_secs(60))
                    .with_max_requests(100),
            )
            .wrap(Logger::default())
    })
    .bind_openssl("127.0.0.1:8080", builder)?
    .run();

    println!("Server running");

    server.await
}
