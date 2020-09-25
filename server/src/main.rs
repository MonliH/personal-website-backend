use actix_web::{get, web, App, HttpServer, HttpRequest};
use actix_files::{Files, NamedFile};

use std::io;

async fn home(_req: HttpRequest) -> io::Result<NamedFile> {
    Ok(NamedFile::open("www/build/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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
