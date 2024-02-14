use crate::repositories::diary_repository::DiaryRepository;
use actix_web::{
    web::{Data},
    HttpResponse,
};
use actix_web::web::{Json, Path};
use crate::models::traits::generic_traits::SetDateTime;


pub  async fn add_sub_diary<Diary> (db: Data<DiaryRepository<Diary>>, path: Path<String>, mut inserted_entry: Json<Diary>) -> HttpResponse where Diary: serde::Serialize + SetDateTime   {
    let id = path.into_inner();
    inserted_entry.set_date_time();
    let entry_detail = db.insert_sub_diary(id.clone(), inserted_entry.into_inner());
    println!("{}", id);
    match entry_detail.await {
        Ok(entry) => HttpResponse::Ok().json(entry),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}