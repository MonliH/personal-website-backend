use actix_web::{get, App, HttpServer, HttpRequest, Responder};
use actix_files;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(actix_files::Files::new("/*", "home/build/").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
        .run()
        .await
}
