use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use crate::models::traits::date_time_trait::SetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub  struct Person {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub  id: Option<ObjectId>,
    pub  name: String,
    pub created_at: Option<mongodb::bson::DateTime>,
}

impl SetDateTime for Person {
    fn set_date_time(&mut self) {
        if self.created_at.is_none() {
            self.created_at = Some(DateTime::now());
        }
    }
}