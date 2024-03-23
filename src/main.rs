use std::{collections::HashMap, sync::{Arc, Mutex}};

use actix_web::{ 
    error::ErrorNotFound
    , web
    , App
    , HttpResponse
    , HttpServer
    , Responder
    , Error
    , get
    , post
    , main
};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User {
    name: String
}

type UserDb = Arc<Mutex<HashMap<u32, User>>>;

#[get("/users/{id}")]
async fn get_user(
    user_id: web::Path<u32>
    , db: web::Data<UserDb>
) -> Result<impl Responder, Error> {
    let user_id = user_id.into_inner();
    let db = db.lock().unwrap();

    match db.get(&user_id) {
        Some(user_data) => Ok(HttpResponse::Ok().json(user_data)),
        None => Err(ErrorNotFound("User not found")),
    }
}

#[get("/users")]
async fn get_users( db: web::Data<UserDb> ) -> impl Responder {
    let db = db.lock().unwrap();

    let users: Vec<&User> = db.values().collect();

    HttpResponse::Ok().json(users)
}

#[derive(Serialize, Deserialize)]
struct CreateUSerResponse {
    id: u32
    , name: String
}

#[post("/users")]
async fn create_user(
    user_data: web::Json<User>
    , db: web::Data<UserDb>
) -> impl Responder {
        let mut db = db.lock().unwrap();
        let new_id = db.keys().max().unwrap_or(&0) + 1;
        let name = user_data.name.clone();

        db.insert(new_id, user_data.into_inner());
        
        HttpResponse::Created().json(CreateUSerResponse {
            id: new_id,
            name
        })
}

#[main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Server running on port {}", port);

    let user_db: UserDb = Arc::new(Mutex::new(HashMap::<u32, User>::new()));

    HttpServer::new(move || {
        let app_data = web::Data::new(user_db.clone());
        App::new().app_data(app_data)
        .service(get_user)
        .service(create_user)
        .service(get_users)
    })
        .bind(("127.0.0.1", port))?
        .workers(2)
        .run()
        .await
}
