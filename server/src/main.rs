use actix_web::{get, App, HttpServer, HttpRequest};
use actix_files::{Files, NamedFile};

use std::io;

#[get("/")]
async fn home(_req: HttpRequest) -> io::Result<NamedFile> {
    Ok(NamedFile::open("home/build/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(Files::new("/*", "home/build/").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
        .run()
        .await
}
