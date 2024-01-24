use mongodb::bson::{DateTime};
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use crate::models::traits::generic_traits::{AddSubIds, SetDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub  struct Diary {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub  id: Option<ObjectId>,
    pub title: String,
    pub date: Option<mongodb::bson::DateTime>,
    pub created_at: Option<mongodb::bson::DateTime>,
    pub content: String,
    pub username: String,
    pub lang: String,
    pub sub: Option<Vec<Box<Diary>>>
}

impl SetDateTime for Diary {
    fn set_date_time(&mut self) {
        if self.created_at.is_none() {
            self.created_at = Some(DateTime::now());
        }
    }
}

impl AddSubIds for Diary {
    fn add_sub_ids(&mut self) {
        let mut arr :Vec<Box<Diary>> = Vec::new();
        if  (self.sub == None) {
            self.sub = Some(arr);
        } 
        if  self.id.is_none() {
            self.id = Some(ObjectId::new());
        }
    }
}