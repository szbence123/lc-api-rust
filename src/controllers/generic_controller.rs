use crate::repositories::generic_repository::GenericRepository;
use actix_web::{
    web::{Data, Path, Json},
    HttpResponse,
};
use serde::de::DeserializeOwned;
use crate::models::traits::generic_traits::SetDateTime;


pub  async  fn get_all<T>(db: Data<GenericRepository<T>>) -> HttpResponse where T: serde::Serialize + DeserializeOwned + Sync + Unpin + Send {
    let entries = db.get_all().await;
    match entries { 
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub  async  fn get_by_id<T>(db: Data<GenericRepository<T>>, path: Path<String>) -> HttpResponse where T: serde::Serialize + DeserializeOwned + Sync + Unpin + Send {
    let id = path.into_inner();
    
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let entry = db.get_by_id(id).await;
    
    match entry {
        Ok(entry) => HttpResponse::Ok().json(entry),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub  async fn add<T>(db: Data<GenericRepository<T>>, new_entry: Json<T>) -> HttpResponse where T: serde::Serialize + DeserializeOwned + Sync + Unpin + Send + SetDateTime {
    let entry_detail = db.add_entry(new_entry.into_inner()).await;
    match entry_detail { 
        Ok(entry) => HttpResponse::Ok().json(entry),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn edit<T>(db: Data<GenericRepository<T>>, path: Path<String> , updated_entry: Json<T>) -> HttpResponse where T: serde::Serialize + DeserializeOwned + Sync + Unpin + Send  {
    let full_path = path.split("/");
    let id = full_path.last().clone().unwrap().to_string();
    let new_entry = db.edit_entry(id, updated_entry.into_inner()).await;
    match new_entry {
        Ok(entry) => HttpResponse::Ok().json(entry),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

pub async fn delete_by_id<T>(db: Data<GenericRepository<T>>, path: Path<String>) -> HttpResponse where T: serde::Serialize + DeserializeOwned + Sync + Unpin + Send  {
    let full_path = path.split("/");
    let id = full_path.last().clone().unwrap().to_string();
    let deleted = db.delete_by_id(id).await;
    match deleted {
        Ok(deleted) => HttpResponse::Ok().json(deleted),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}