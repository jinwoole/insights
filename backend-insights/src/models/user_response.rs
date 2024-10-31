// src/models/user_response.rs

use serde::Serialize;
use bson::oid::ObjectId;

#[derive(Debug, Serialize)]
pub struct UserResponse {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
}
