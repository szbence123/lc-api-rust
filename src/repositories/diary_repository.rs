use std::env;
use std::str::FromStr;

extern crate dotenv;
use dotenv::dotenv;

use mongodb::{Client, Collection};
use mongodb::{bson::{doc, oid::ObjectId}, error::Error, results::UpdateResult};


pub struct DiaryRepository<Diary> {
    col: Collection<Diary>,
}

impl<Diary> DiaryRepository<Diary> where Diary:  serde::Serialize {
    pub async fn init(db_name: String, db_col: String) -> Self {
        dotenv().ok();
        let uri = match env::var("DATABASE_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database(&db_name);
        let col: Collection<Diary> = db.collection(&db_col);
        DiaryRepository { col }
    }

    pub async fn insert_sub_diary(&self, parent_diary_id: String, sub_diary: Diary) -> Result<UpdateResult, Error> where Diary:  serde::Serialize  {
        let parent_id = ObjectId::from_str(&parent_diary_id).expect("Error converting to ObjectId");
        // Build the update query
        let filter = doc! { "_id": parent_id };
        
        let _er = self.col.find(filter.clone(), None).await.map_err(|e| println!("{}", e)).ok().expect("");
        
        let update = doc! { "$push": { "sub": mongodb::bson::to_document(&sub_diary).expect("Error") } };

        // Execute the update
        let inserted = self.col.update_one(filter, update, None).await.map_err(|e| println!("{}", e)).ok().expect("Error while adding subdiary");
        Ok(inserted)
    }
}