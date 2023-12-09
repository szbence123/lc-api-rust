mod models;
mod repositories;
mod controllers;

use actix_web::{web::Data, App, HttpServer, web};
use controllers::generic_controller::{get_all, get_by_id, edit, delete_by_id, add};
use controllers::diary_controller::{add_sub_diary};
use models::diary_model::Diary;
use models::dream_model::Dream;
use models::person_model::Person;
use repositories::generic_repository::GenericRepository;
use repositories::diary_repository::DiaryRepository;
#[actix_web::main]

async fn main() -> std::io::Result<()> {
    let person_rep = GenericRepository::<Person>::init("LeehCross".to_string(), "Person".to_string()).await;
    let db_person_data: Data<GenericRepository<Person>> = Data::new(person_rep);


    let diary_rep_gen = GenericRepository::<Diary>::init("LeehCross".to_string(), "Diaries".to_string()).await;
    let db_diary_data_gen: Data<GenericRepository<Diary>> = Data::new(diary_rep_gen);

    let dream_rep= GenericRepository::<Dream>::init("LeehCross".to_string(), "Dreams".to_string()).await;
    let db_dream_data: Data<GenericRepository<Dream>> = Data::new(dream_rep);

    let diary_rep = DiaryRepository::<Diary>::init("LeehCross".to_string(), "Diaries".to_string()).await;
    let db_diary_data: Data<DiaryRepository<Diary>> = Data::new(diary_rep);

    HttpServer::new(move || {
        App::new()
            .app_data(db_person_data.clone())
            /* PERSON COLLECTION */
            .route("/person/get_all",       web::get()    .to(get_all::       <Person>))
            .route("/person/get/{id}",      web::get()    .to(get_by_id::     <Person>))
            .route("/person/add",           web::post()   .to(add::           <Person>))
            .route("/person/edit/{id}",     web::put()    .to(edit::          <Person>))
            .route("/person/delete/{id}",   web::delete() .to(delete_by_id::  <Person>))
            /* DIARY COLLECTION */
            .app_data(db_diary_data.clone())
            .route("/diary/insert_sub/{id}",      web::put()     .to(add_sub_diary::  <Diary>))
            .app_data(db_diary_data_gen.clone())
            .route("/diary/get_all",        web::get()    .to(get_all::       <Diary>))
            .route("/diary/get/{id}",       web::get()    .to(get_by_id::     <Diary>))
            .route("/diary/add",            web::post()   .to(add::           <Diary>))
            .route("/diary/edit/{id}",      web::put()    .to(edit::          <Diary>))
            .route("/diary/delete/{id}",    web::delete() .to(delete_by_id::  <Diary>))
            /* DREAM COLLECTION */
            .app_data(db_dream_data.clone())
            .route("/dream/get_all",    web::get()    .to(get_all::       <Dream>))
            .route("/dream/get/{id}",   web::get()    .to(get_by_id::     <Dream>))
            .route("/dream/add",        web::post()   .to(add::           <Dream>))
            .route("/dream/edit/{id}",  web::put()    .to(edit::          <Dream>))
            .route("/dream/delete/{id}",web::delete() .to(delete_by_id::  <Dream>))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
