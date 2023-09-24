mod models;
mod repositories;
mod controllers;

use actix_web::{web::Data, get, App, HttpResponse, HttpServer, Responder};
use controllers::person_controller::{add_person, get_all_person, get_person_by_id};
use repositories::person_repository::PersonRepository;

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().json("Hello world")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = PersonRepository::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(add_person)
            .service(get_all_person)
            .service(get_person_by_id)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
