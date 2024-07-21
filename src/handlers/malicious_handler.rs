use crate::models::MaliciousUrl;
use actix_web::{web, HttpResponse, Responder};
use futures::stream::StreamExt;
use mongodb::{
    bson::{self, doc},
    options::FindOptions,
    Client, Collection,
};

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

// Get all blocked URLs
pub async fn get_all_blacklist_url(db_client: web::Data<Client>) -> impl Responder {
    let collection: Collection<MaliciousUrl> = db_client
        .database("rustkeeper")
        .collection("malicious_urls");

    let filter = bson::doc! { "status": "blocked" };
    let find_options = FindOptions::builder()
        .sort(bson::doc! { "created_at": -1 })
        .build();
    let mut cursor = match collection.find(filter, find_options).await {
        Ok(cursor) => cursor,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };

    let mut results: Vec<MaliciousUrl> = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => results.push(document),
            Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
        }
    }

    HttpResponse::Ok().json(results)
}
