use diesel::prelude::*;
use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use crate::models::traits::generic_traits::SetDateTime;
#[derive(Debug, Serialize, Deserialize, Queryable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub  struct Dream {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub  id: Option<ObjectId>,
    pub title: String,
    pub text: String,
    pub trigger: String,
    pub lucidity5: i32,
    pub remember5: i32,
    pub created_at: Option<mongodb::bson::DateTime>,
}

impl SetDateTime for Dream {
    fn set_date_time(&mut self) {
        if self.created_at.is_none() {
            self.created_at = Some(DateTime::now());
        }
    }
}