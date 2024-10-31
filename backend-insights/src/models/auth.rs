// src/models/auth.rs

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}
