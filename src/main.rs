mod models;
mod repositories;
mod controllers;

use actix_web::{web::Data, App, HttpServer, web};
use controllers::generic_controller::{get_all, get_by_id, edit, delete_by_id, add};
use controllers::person_controller::{add_person, get_all_person, get_person_by_id, delete_person_by_id, edit_person};
use models::diary_model::Diary;
use models::dream_model::Dream;
use repositories::person_repository::PersonRepository;
use repositories::generic_repository::GenericRepository;
#[actix_web::main]

async fn main() -> std::io::Result<()> {
    let db = PersonRepository::init().await;
    let db_data = Data::new(db);

    let diary_rep = GenericRepository::<Diary>::init("LeehCross".to_string(), "Diaries".to_string()).await;
    let db_diary_data: Data<GenericRepository<Diary>> = Data::new(diary_rep);

    let dream_rep= GenericRepository::<Dream>::init("LeehCross".to_string(), "Dreams".to_string()).await;
    let db_dream_data: Data<GenericRepository<Dream>> = Data::new(dream_rep);

    HttpServer::new(move || {
        App::new()
            /* PERSON COLLECTION */
            .app_data(db_data.clone())
            .service(add_person)
            .service(get_all_person)
            .service(get_person_by_id)
            .service(delete_person_by_id)
            .service(edit_person)
            /* DIARY COLLECTION */
            .app_data(db_diary_data.clone())
            .route("/{route:.*}", web::get()    .to(get_all::       <Diary>))
            .route("/{route:.*}", web::get()    .to(get_by_id::     <Diary>))
            .route("/{route:.*}", web::post()   .to(add::           <Diary>))
            .route("/{route:.*}", web::put()    .to(edit::          <Diary>))
            .route("/{route:.*}", web::delete() .to(delete_by_id::  <Diary>))
            /* DREAM COLLECTION */
            .app_data(db_dream_data.clone())
            .route("/{route:.*}", web::get()    .to(get_all::       <Dream>))
            .route("/{route:.*}", web::get()    .to(get_by_id::     <Dream>))
            .route("/{route:.*}", web::post()   .to(add::           <Dream>))
            .route("/{route:.*}", web::put()    .to(edit::          <Dream>))
            .route("/{route:.*}", web::delete() .to(delete_by_id::  <Dream>))

    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
