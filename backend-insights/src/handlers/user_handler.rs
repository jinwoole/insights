use actix_web::{get, web, HttpResponse, Responder};
use mongodb::{Client, bson::doc};
use crate::models::user::User;
use crate::middleware::auth_middleware::AuthorizedUser;

#[get("/get")]
pub async fn get_user(
    client: web::Data<Client>,
    user: AuthorizedUser,
) -> impl Responder {
    let users = client.database("insights").collection::<User>("users");
    match users.find_one(doc! { "_id": &user.id }, None).await {
        Ok(Some(user_data)) => HttpResponse::Ok().json(serde_json::json!({ "username": user_data.username, "email": user_data.email })),
        Ok(None) => HttpResponse::NotFound().json("User not found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

