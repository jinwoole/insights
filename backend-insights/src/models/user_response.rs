// src/models/user_response.rs

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserResponse {
    #[serde(rename = "_id")]
    pub username: String,
}
