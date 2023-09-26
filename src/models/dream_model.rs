use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub  struct Dream {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub  id: Option<ObjectId>,
    pub title: String,
    pub text: String,
    pub trigger: String,
    pub lucidity5: i32,
    pub remember5: i32
}