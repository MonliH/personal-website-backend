#[macro_use]
extern crate lazy_static;

mod admin;
mod admin_blog;
mod blog;
mod db;
mod embeds;
mod public_blog;
mod submission;

use db::{DBState, DB};

use actix_cors::Cors;
use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
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
            .wrap(
                RateLimiter::new(MemoryStoreActor::from(store.clone()).start())
                    .with_interval(Duration::from_secs(60))
                    .with_max_requests(100),
            )
            .wrap(if env::var("DEVELOPMENT_TEST").is_ok() {
                Cors::permissive()
            } else {
                Cors::default()
                    .allowed_origin(
                        &env::var("FRONTEND_DOMAIN").expect("No FRONTEND_DOMAIN variable found"),
                    )
                    .allowed_origin(
                        &env::var("FRONTEND_DOMAIN_ALT")
                            .expect("No FRONTEND_DOMAIN_ALT variable found"),
                    )
            })
            .service(admin_blog::admin_delete)
            .service(admin_blog::admin_edits)
            .service(admin_blog::admin_key)
            .service(public_blog::blog_post_display_by_name)
            .service(public_blog::blog_post_admin_by_name)
            .service(public_blog::number_of_blog_posts)
            .service(public_blog::blog_entries)
            .service(public_blog::all_blog_urls)
            .service(submission::new_contact)
            .service(submission::number_of_contacts)
            .service(submission::contact_range)
            .service(embeds::discord_embed_json)
            .service(embeds::simple_embed)
    })
    .bind(env::var("HOST_IP_HTTP").expect("No HOST_IP variable found"))?
    .run();

    println!("Server running");

    server.await
}
