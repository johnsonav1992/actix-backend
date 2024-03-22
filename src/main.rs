use std::{collections::HashMap, sync::{Arc, Mutex}};

use actix_web::{ App, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    name: String
}

type UserDb = Arc<Mutex<HashMap<u32, User>>>;

#[actix_web::get("/greet/{id}")]
async fn greet(user_id: web::Path<u32>) -> impl Responder {
    format!("Hey there, {}!", user_id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Server running on port {}", port);

    let user_db: UserDb = Arc::new(Mutex::new(HashMap::<u32, User>::new()));

    HttpServer::new(move || {
        let app_data = web::Data::new(user_db.clone());
        App::new().service(greet)
    })
        .bind(("127.0.0.1", port))?
        .workers(2)
        .run()
        .await
}
