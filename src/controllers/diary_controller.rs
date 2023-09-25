use crate::repositories::generic_repository::GenericRepository;
use crate::models::diary_model::Diary;
use actix_web::{
get, post, put, delete,
    web::{Data, Path, Json},
    HttpResponse,
};
#[get("/diary/get_all")]
pub  async  fn get_all_diaries(db: Data<GenericRepository<Diary>>) -> HttpResponse {
    let diaries = db.get_all().await;
    match diaries { 
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[get("/diary/get/{id}")]
pub  async  fn get_diary_by_id(db: Data<GenericRepository<Diary>>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let person = db.get_by_id(id).await;
    
    match person {
        Ok(person) => HttpResponse::Ok().json(person),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/diary/add")]
pub  async fn add_diary(db: Data<GenericRepository<Diary>>, mut new_diary: Json<Diary>) -> HttpResponse {
    new_diary.id = None;
    let person_detail = db.add_entry(new_diary.into_inner()).await;
    match person_detail { 
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/diary/edit/{id}")]
pub async fn edit_diary(db: Data<GenericRepository<Diary>>, path: Path<String> , mut updated_diary: Json<Diary>) -> HttpResponse {
    let id = path.into_inner();
    let new_diary = db.edit_person(id, updated_diary.into_inner()).await;
    match new_diary {
        Ok(diary) => HttpResponse::Ok().json(diary),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[delete("/diary/delete/{id}")]
pub async fn delete_diary_by_id(db: Data<GenericRepository<Diary>>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let deleted = db.delete_by_id(id).await;

    match deleted {
        Ok(deleted) => HttpResponse::Ok().json(deleted),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}