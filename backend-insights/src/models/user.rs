// src/models/user.rs

use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId, // ObjectId로 정의
    pub username: String,
    pub email: String, // `skip_serializing` 제거
}
