use crate::{models::person_model::Person, repositories::person_repository::PersonRepository};
use actix_web::{
    post, get, put, delete,
    web::{Data, Json, Path},
    HttpResponse,
};
#[get("/person/get_all")]
pub  async  fn get_all_person(db: Data<PersonRepository>) -> HttpResponse {
    let people = db.get_all().await;
    match people { 
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/person/get/{id}")]
pub  async  fn get_person_by_id(db: Data<PersonRepository>, path: Path<String>) -> HttpResponse {
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

#[post("/person/add")]
pub  async fn add_person(db: Data<PersonRepository>, new_person: Json<Person>) -> HttpResponse {
    let data = Person {
        id: None,
        name: new_person.name.to_owned(),
        medsnoen: new_person.medsnoen.to_owned()
    };
    let person_detail = db.add_person(data).await;
    match person_detail { 
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/person/edit/{id}")]
pub async fn edit_person(db: Data<PersonRepository>, path: Path<String> , updated_person: Json<Person>) -> HttpResponse {
    let id = path.into_inner();
    let data = Person {
        id: updated_person.id.to_owned(),
        name: updated_person.name.to_owned(),
        medsnoen: updated_person.medsnoen.to_owned()
    };
    let new_person = db.edit_person(id, data).await;
    match new_person {
        Ok(person) => HttpResponse::Ok().json(person),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[delete("/person/delete/{id}")]
pub async fn delete_person_by_id(db: Data<PersonRepository>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let deleted = db.delete_by_id(id).await;

    match deleted {
        Ok(deleted) => HttpResponse::Ok().json(deleted),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }

}