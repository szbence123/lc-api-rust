mod models;
mod repositories;
mod controllers;

use actix_web::{web::Data, App, HttpServer, web};
use controllers::generic_controller::{get_all, get_by_id, edit, delete_by_id, add};
use models::diary_model::Diary;
use models::dream_model::Dream;
use models::person_model::Person;
use repositories::generic_repository::GenericRepository;
#[actix_web::main]

async fn main() -> std::io::Result<()> {
    let person_rep = GenericRepository::<Person>::init("LeehCross".to_string(), "Person".to_string()).await;
    let db_person_data: Data<GenericRepository<Person>> = Data::new(person_rep);


    let diary_rep = GenericRepository::<Diary>::init("LeehCross".to_string(), "Diaries".to_string()).await;
    let db_diary_data: Data<GenericRepository<Diary>> = Data::new(diary_rep);

    let dream_rep= GenericRepository::<Dream>::init("LeehCross".to_string(), "Dreams".to_string()).await;
    let db_dream_data: Data<GenericRepository<Dream>> = Data::new(dream_rep);

    HttpServer::new(move || {
        App::new()
            .app_data(db_person_data.clone())
            /* PERSON COLLECTION */
            .route("/{route:.*}", web::get()    .to(get_all::       <Person>))
            .route("/{route:.*}", web::get()    .to(get_by_id::     <Person>))
            .route("/{route:.*}", web::post()   .to(add::           <Person>))
            .route("/{route:.*}", web::put()    .to(edit::          <Person>))
            .route("/{route:.*}", web::delete() .to(delete_by_id::  <Person>))
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
