#[macro_use]
extern crate lazy_static;

use actix_web::{get, web, App, HttpServer, HttpRequest, HttpResponse};
use actix_files::{Files, NamedFile};

use std::io;
use std::fs::{File, read_dir};

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

    static ref BLOGS_YAML: Vec<String> = {
        BLOGS.0
            .iter()
            .map(|post|
                serde_yaml::to_string(post)
                .expect("Error encoding yaml")
            )
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
        .content_type("application/x-yaml")
        .body(if ending > BLOGS_YAML.len() {
            // Out of range
            return HttpResponse::BadRequest().body("specified range out of range");
        } else {
            BLOGS_YAML[starting..ending].join("\n...\n")
        })
}

#[get("/api/blog/pages")]
async fn blog_post_amounts(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(LENGTH.clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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
