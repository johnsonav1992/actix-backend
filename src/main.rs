use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;

    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", port))?
        .workers(2)
        .run()
        .await
}
