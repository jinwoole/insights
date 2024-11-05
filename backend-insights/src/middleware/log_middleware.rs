use actix_web::{
    web,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{LocalBoxFuture, Ready, ready};
use std::rc::Rc;
use std::task::{Context, Poll};
use crate::errors::AppError;

use mongodb::Client;
use mongodb::bson::doc;

// Define the LogMiddleware struct
pub struct LogMiddleware;

// Implement the Transform trait for LogMiddleware
impl<S, B> Transform<S, ServiceRequest> for LogMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LogMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LogMiddlewareMiddleware {
            service: Rc::new(service),
        }))
    }
}

// Define the middleware structure
pub struct LogMiddlewareMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for LogMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let method = req.method().clone();
        let path = req.path().to_string();

        Box::pin(async move {
            // Proceed to the next service first
            let res = service.call(req).await?;

            // Attempt to retrieve the MongoDB client from app data
            if let Some(client_data) = res.request().app_data::<web::Data<Client>>() {
                let client = client_data.as_ref();
                let logs_collection = client.database("insights").collection("logs");

                // Create TTL index on "timestamp" field with 1 year expiration
                let ttl_index = mongodb::IndexModel::builder()
                    .keys(doc! { "timestamp": 1 })
                    .options(mongodb::options::IndexOptions::builder()
                        .expire_after(Some(31536000)) // 1 year in seconds
                        .build())
                    .build();
                let _ = logs_collection.create_index(ttl_index, None).await;

                let log_entry = doc! {
                    "method": method.to_string(),
                    "path": path,
                    "status": res.status().as_u16() as i32,
                    "timestamp": bson::DateTime::now(),
                };
                // Insert the log entry into the "logs" collection
                if let Err(e) = logs_collection.insert_one(log_entry, None).await {
                    println!("Failed to insert log entry: {}", e);
                }
            } else {
                println!("MongoDB client not found in app data.");
            }
            Ok(res)
        })
    }
}
