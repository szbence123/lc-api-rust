/* 
_id
62067b2fa341b2aabf132593
guid
"636b63e4-f660-4c6a-9377-04e3a4f1dbe3"
title
"Vayvennoer"
date
2022-02-08T23:00:00.000+00:00
content
"{"time":1644797521493,"blocks":[{"id":"pVnmSZCzLJ","type":"header","da…"
username
"Blaeksowl"
lang
"pol"
planet
"closer"
topic
"Common"

 */
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
    pub topic: String
}