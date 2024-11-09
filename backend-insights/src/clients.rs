// src/clients.rs

use mongodb::{options::ClientOptions, Client as MongoClient};
use std::env;

pub async fn init_db() -> mongodb::error::Result<MongoClient> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let client_options = ClientOptions::parse(&database_url).await?;
    let db_client = MongoClient::with_options(client_options)?;
    Ok(db_client)
}

use aws_credential_types::Credentials;
use aws_types::region::Region;
use aws_sdk_sesv2::{Config, Client as SesClient, Error};

pub async fn init_ses_client() -> Result<SesClient, Error> {
    let access_key_id = env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID must be set");
    let secret_access_key = env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set");

    let credentials = Credentials::from_keys(&access_key_id, &secret_access_key, None);

    let config = Config::builder()
        .credentials_provider(credentials)
        .region(Region::new("ap-northeast-2"))
        .build();

    let client = SesClient::from_conf(config);
    Ok(client)
}