use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{ InsertOneResult },
    Cursor,
    Client, Collection,
};
use futures::stream::{StreamExt, TryStreamExt};

use crate::models::person_model::Person;
pub struct PersonRepository {
    col: Collection<Person>,
}

impl PersonRepository {
    pub  async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("DATABASE_URI") { 
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("LeehCross");
        let col: Collection<Person> = db.collection("Person");
        PersonRepository { col }
    }
    
    pub  async fn get_all(&self) ->  Result<Vec<Person>, Error> {
        let mut cursors = self.col.find(None, None).await.ok().expect("Error");
        
        let mut users: Vec<Person> = Vec::new();
        
        while let Some(user) = cursors.try_next().await.ok().expect("Error mapping through cursor") {
            users.push(user)
        }
        
        Ok(users)
    }
    
    pub  async fn get_by_id(&self, id: String) -> Result<Person, Error> {
        let id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": id };
        let person = self.col.find_one(filter, None).await.ok().expect("Error");
        Ok(person.unwrap())
    }
    
    pub  async fn add_person(&self, new_person: Person) -> Result<InsertOneResult, Error> {
        let new_doc = Person {
            id: None,
            name: new_person.name,
            medsnoen: new_person.medsnoen,
        };
        
        let person = self.col.insert_one(new_doc, None).await.ok().expect("Error");
        Ok(person)
    }
}

