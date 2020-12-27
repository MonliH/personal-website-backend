mod admin;
mod blog;
mod db;
mod embeds;
mod public_blog;
mod submission;

use db::{DBState, DB};

use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_cors::Cors;
use actix_web::{App, HttpServer};

use dotenv::dotenv;
use env_logger::Env;

use std::env;
use std::sync::Arc;
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let db = Arc::new(DB::new().await.expect("Failed to get DB handle"));

    let store = MemoryStore::new();

    let server = HttpServer::new(move || {
        App::new()
            .data(Arc::clone(&db) as DBState)
            .service(admin::admin_delete)
            .service(admin::admin_edits)
            .service(admin::admin_key)
            .service(public_blog::blog_post_by_name)
            .service(public_blog::blog_post_amounts)
            .service(public_blog::blog_entries)
            .service(public_blog::all_blog_urls)
            .service(embeds::discord_embed_json)
            .service(embeds::simple_embed)
            .wrap(
                RateLimiter::new(MemoryStoreActor::from(store.clone()).start())
                    .with_interval(Duration::from_secs(60))
                    .with_max_requests(100),
            )
            .wrap(
                Cors::default()
                    .allowed_origin(
                        &env::var("FRONTEND_DOMAIN").expect("No FRONTEND_DOMAIN variable found")
                    )
                    .allowed_origin(
                        &env::var("FRONTEND_DOMAIN_ALT").expect("No FRONTEND_DOMAIN_ALT variable found")
                    )
            )
    })
    .bind(env::var("HOST_IP_HTTP").expect("No HOST_IP variable found"))?
    .run();

    println!("Server running");

    server.await
}
