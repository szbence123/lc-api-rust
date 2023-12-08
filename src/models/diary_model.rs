use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub  struct Diary {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub  id: Option<ObjectId>,
    pub title: String,
    pub date: mongodb::bson::DateTime,
    pub content: String,
    pub username: String,
    pub lang: String,
    pub sub: Option<Vec<Box<Diary>>>
}