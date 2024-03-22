use actix_web::{ App, HttpServer, Responder};

#[actix_web::get("/greet")]
async fn greet() -> impl Responder {
    format!("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Server running on port {}", port);

    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", port))?
        .workers(2)
        .run()
        .await
}
