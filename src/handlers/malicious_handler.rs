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

// Define a helper struct for deserializing incoming data
#[derive(Debug, Deserialize)]
pub struct UpdateInputData {
    pub url: String,
    pub status: String,
}

// Update a malicious url by ID
pub async fn edit_blacklist_url_by_id(
    db_client: web::Data<Client>,
    path: web::Path<String>,
    data: web::Json<UpdateInputData>,
) -> impl Responder {
    let collection: Collection<MaliciousUrl> = db_client
        .database("rustkeeper")
        .collection("malicious_urls");

    let id_str = path.into_inner();
    let oid = match ObjectId::parse_str(&id_str) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID format"),
    };

    let update = doc! {
        "$set": {
            "url": &data.url,
            "status": &data.status,
            "updated_at": bson::DateTime::now(),  // Automatically update the 'updated_at' field
        }
    };

    match collection
        .update_one(doc! { "_id": oid }, update, None)
        .await
    {
        Ok(update_result) => {
            if update_result.modified_count == 1 {
                HttpResponse::Ok().json("Malicious URL successfully updated")
            } else {
                HttpResponse::NotFound()
                    .body("No entry found with the provided ID or no changes made")
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

// Check if URL is in the blacklist
pub async fn is_blacklist_url(
    db_client: web::Data<Client>,
    data: web::Json<InputData>,
) -> impl Responder {
    println!("Received request to check URL: {:?}", data.url); // Add logging

    let collection: Collection<MaliciousUrl> = db_client
        .database("rustkeeper")
        .collection("malicious_urls");

    // Create a different filter for the root URL
    let filter = if data.url == "/" {
        doc! {
            "url": "/",
            "status": "blocked"
        }
    } else {
        doc! {
            "$or": [
                { "url": &data.url },
                { "url": { "$regex": &format!("{}.*", &data.url), "$options": "i" } },
            ],
            "status": "blocked"
        }
    };

    println!("Query filter: {:?}", filter); // Add logging

    match collection.find_one(filter, None).await {
        Ok(Some(result)) => {
            println!("URL is blacklisted: {:?}", result); // Add logging
            HttpResponse::Ok().json(true) // URL is blacklisted
        }
        Ok(None) => {
            println!("URL is not blacklisted: {:?}", data.url); // Add logging
            HttpResponse::Ok().json(false) // URL is not blacklisted
        }
        Err(e) => {
            println!("Error checking blacklist: {}", e); // Add logging
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}
