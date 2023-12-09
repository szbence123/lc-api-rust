use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{ InsertOneResult, UpdateResult} ,
    Client, Collection, Cursor,
};
use futures::stream::TryStreamExt;
use serde::de::DeserializeOwned;
use crate::models::traits::date_time_trait::SetDateTime;

pub struct GenericRepository<T> {
    col: Collection<T>,
}


impl<T> GenericRepository<T> where T:  DeserializeOwned + Unpin + Send + Sync + serde::Serialize {
    pub  async fn init(db_name: String, db_col: String) -> Self {
        dotenv().ok();
        let uri = match env::var("DATABASE_URI") { 
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database(&db_name);
        let col: Collection<T> = db.collection(&db_col);
        GenericRepository { col }
    }
    
    pub  async fn get_all(&self) ->  Result<Vec<T>, Error> {
        let mut cursors: Cursor<T> = self.col.find(None, None).await.ok().expect("Error");
        let mut entries: Vec<T> = Vec::new();
        while let Some(entry) = cursors.try_next().await.map_err(|e| println!("{}", e)).ok().expect("Error mapping through cursor") {
            entries.push(entry)
        }
        Ok(entries)
    }
    
    pub  async fn get_by_id(&self, id: String) -> Result<T, Error> {
        let id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": id };
        let entry = self.col.find_one(filter, None).await.map_err(|e| println!("{}", e)).ok().expect("Error");
        Ok(entry.unwrap())
    }
    
    pub  async fn add_entry(&self, mut new_entry: T) -> Result<InsertOneResult, Error> where T: SetDateTime {
        new_entry.set_date_time();
        let entry = self.col.insert_one(new_entry, None).await.map_err(|e| println!("{}", e)).ok().expect("Error");
        Ok(entry)
    }

    pub async fn edit_entry(&self, id: String, updated_entry: T) -> Result<UpdateResult, Error> {
        let id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": id };
        let doc = doc! {
            "$set": mongodb::bson::to_document(&updated_entry).expect("Error")
        };
        let new_person = self.col.update_one(filter, doc, None).await.ok().expect("Error");
        Ok(new_person)
    }

    pub async fn delete_by_id(&self, id: String) -> Result<String, Error> {
        let id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": id};
        self.col.delete_one(filter, None).await.ok().expect("Error");
        Ok(format!("{} deleted", id))
    }
    
}

