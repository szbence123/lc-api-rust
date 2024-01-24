use crate::repositories::diary_repository::DiaryRepository;
use actix_web::{
    web::{Data},
    HttpResponse,
};
use actix_web::web::{Json, Path};
use crate::models::traits::generic_traits::AddSubIds;


pub  async fn add_sub_diary<Diary> (db: Data<DiaryRepository<Diary>>, path: Path<String>, inserted_entry: Json<Diary>) -> HttpResponse where Diary: serde::Serialize + AddSubIds  {
    let id = path.into_inner();
    let entry_detail = db.insert_sub_diary(id.clone(), inserted_entry.into_inner());
    println!("{}", id);
    match entry_detail.await {
        Ok(entry) => HttpResponse::Ok().json(entry),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}