#[macro_use]
extern crate lazy_static;

use actix_web::{get, web, App, HttpServer, HttpRequest, HttpResponse};
use actix_files::{Files, NamedFile};

use std::io;
use std::fs::{File, read_dir};
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Serialize, Deserialize)]
struct Blog {
    title: String,
    contents: String,
    date: NaiveDate,
    url: String,
}

struct Blogs(Vec<Blog>);

lazy_static! {
    static ref BLOGS: Blogs = {
        let mut blogs = read_dir("assets/blog/entries/")
            .unwrap()
            .map(|entry_path|
                serde_yaml::from_reader(
                    File::open(entry_path.unwrap().path()).expect("File does not exist\nWas it deleted?")
                ).expect("File was not valid yaml"))
            .collect::<Vec<Blog>>();
        blogs.sort_unstable_by_key(|blog| blog.date);

        Blogs(blogs)
    };

    static ref BLOGS_JSON: Vec<String> = {
        BLOGS.0
            .iter()
            .map(|post|
                serde_json::to_string(post)
                .expect("Error encoding json")
            )
            .collect()
    };

    static ref BLOGS_JSON_MAP: HashMap<String, String> = {
        BLOGS.0
            .iter()
            .zip(BLOGS_JSON.iter())
            .map(|(post, json)| (post.url.clone(), json.clone()))
            .collect()
    };

    static ref LENGTH: String = {
        BLOGS.0.len().to_string()
    };
}

async fn home(_req: HttpRequest) -> io::Result<NamedFile> {
    Ok(NamedFile::open("www/build/index.html")?)
}

#[get("/api/blog/entries/{starting}/{ending}")]
async fn blog_entries(web::Path((starting, ending)): web::Path<(usize, usize)>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(if ending > BLOGS_JSON.len() {
            // Out of range
            return HttpResponse::RangeNotSatisfiable().body("specified range out of range");
        } else {
            format!("[{}]", BLOGS_JSON[starting..ending].join(","))
        })
}

#[get("/api/blog/pages")]
async fn blog_post_amounts(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(LENGTH.clone())
}

#[get("/api/blog/entry/{name}")]
async fn blog_post_by_name(web::Path(name): web::Path<String>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(match BLOGS_JSON_MAP.get(&name) {
            Some(post) => post,
            None => {
                return HttpResponse::NotFound().body("post not found");
            }
        })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(blog_post_by_name)
            .service(blog_post_amounts)
            .service(blog_entries)
            .service(
                Files::new("/*", "www/build/")
                .index_file("www/build/index.html")
                .default_handler(web::route().to(home))
            )
    })
    .bind("127.0.0.1:8080")?
        .run()
        .await
}
