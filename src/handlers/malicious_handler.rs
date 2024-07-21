use crate::models::MaliciousUrl;
use actix_web::{web, HttpResponse, Responder};
use mongodb::{bson::doc, Client, Collection};

use serde::Deserialize;

// Define a helper struct for deserializing incoming data
#[derive(Debug, Deserialize)]
pub struct InputData {
    pub url: String,
}

// Post request handler to add a new URL to the blacklist
pub async fn add_blacklist_url(
    db_client: web::Data<Client>,
    data: web::Json<InputData>,
) -> impl Responder {
    let collection: Collection<MaliciousUrl> = db_client
        .database("rustkeeper")
        .collection("malicious_urls");

    // Create a new MaliciousUrl using the helper method that sets timestamps and default status
    let new_url = MaliciousUrl::new(data.url.clone());

    match collection.insert_one(new_url, None).await {
        Ok(_) => HttpResponse::Created()
            .json("URL successfully added to blacklist with status 'blocked'"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
