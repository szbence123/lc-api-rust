mod models;
mod repositories;
mod controllers;

use actix_web::{web::Data, App, HttpServer};
use controllers::diary_controller::{get_all_diaries, get_diary_by_id, add_diary, edit_diary, delete_diary_by_id};
use controllers::person_controller::{add_person, get_all_person, get_person_by_id, delete_person_by_id, edit_person};
use models::diary_model::Diary;
use repositories::person_repository::PersonRepository;
use repositories::generic_repository::GenericRepository;
#[actix_web::main]

async fn main() -> std::io::Result<()> {
    let db = PersonRepository::init().await;
    let db_data = Data::new(db);

    let diary_rep = GenericRepository::<Diary>::init("LeehCross".to_string(), "Diaries".to_string()).await;
    let db_diary_data: Data<GenericRepository<Diary>> = Data::new(diary_rep);


    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(add_person)
            .service(get_all_person)
            .service(get_person_by_id)
            .service(delete_person_by_id)
            .service(edit_person)
            
            .app_data(db_diary_data.clone())
            .service(get_all_diaries)
            .service(get_diary_by_id)
            .service(add_diary)
            .service(edit_diary)
            .service(delete_diary_by_id)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
