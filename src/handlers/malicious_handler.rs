use crate::models::MaliciousUrl;
use actix_web::{web, HttpResponse, Responder};
use futures::stream::StreamExt;
use mongodb::{
    bson::{self, doc, oid::ObjectId},
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

// Get a single blocked URL by ID
pub async fn get_blacklist_url_by_id(
    db_client: web::Data<Client>,
    path: web::Path<String>,
) -> impl Responder {
    let collection: Collection<MaliciousUrl> = db_client
        .database("rustkeeper")
        .collection("malicious_urls");

    let id_str = path.into_inner();
    let oid = match ObjectId::parse_str(&id_str) {
        // Corrected method here
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID format"),
    };

    let filter = doc! { "_id": oid };
    match collection.find_one(filter, None).await {
        Ok(Some(malicious)) => HttpResponse::Ok().json(malicious),
        Ok(None) => HttpResponse::NotFound().body("No entry found with the provided ID"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

// Delete a single blocked URL by ID
pub async fn delete_blacklist_url_by_id(
    db_client: web::Data<Client>,
    path: web::Path<String>,
) -> impl Responder {
    let collection: Collection<MaliciousUrl> = db_client
        .database("rustkeeper")
        .collection("malicious_urls");

    let id_str = path.into_inner();
    let oid = match ObjectId::parse_str(&id_str) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID format"),
    };

    match collection.delete_one(doc! { "_id": oid }, None).await {
        Ok(delete_result) => {
            if delete_result.deleted_count == 1 {
                HttpResponse::Ok().json("Malicious URL successfully deleted")
            } else {
                HttpResponse::NotFound().body("No entry found with the provided ID")
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
