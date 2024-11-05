// src/models/user.rs

use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub email: String,
}
