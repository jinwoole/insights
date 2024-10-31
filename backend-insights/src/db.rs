// src/db.rs

use mongodb::{options::ClientOptions, Client};
use std::env;

pub async fn init_db() -> mongodb::error::Result<Client> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let client_options = ClientOptions::parse(&database_url).await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}
