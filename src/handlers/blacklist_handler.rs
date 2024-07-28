use crate::models::BlacklistedIp;
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
    pub ip_address: String,
}

// Post request handler to add a new IP to the blacklist
pub async fn add_blacklist_ip(
    db_client: web::Data<Client>,
    data: web::Json<InputData>,
) -> impl Responder {
    let collection: Collection<BlacklistedIp> = db_client
        .database("rustkeeper")
        .collection("blacklisted_ips");

    // Create a new BlacklistedIp using the helper method that sets timestamps and default status
    let new_ip = BlacklistedIp::new(data.ip_address.clone());

    match collection.insert_one(new_ip, None).await {
        Ok(_) => {
            HttpResponse::Created().json("IP successfully added to blacklist with status 'blocked'")
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

// Get all blocked IPs
pub async fn get_all_blacklist_ip(db_client: web::Data<Client>) -> impl Responder {
    let collection: Collection<BlacklistedIp> = db_client
        .database("rustkeeper")
        .collection("blacklisted_ips");

    let filter = bson::doc! { "status": "blocked" };
    let find_options = FindOptions::builder()
        .sort(bson::doc! { "created_at": -1 })
        .build();
    let mut cursor = match collection.find(filter, find_options).await {
        Ok(cursor) => cursor,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };

    let mut results: Vec<BlacklistedIp> = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => results.push(document),
            Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
        }
    }

    HttpResponse::Ok().json(results)
}

// Get a single blacklisted IP by ID
pub async fn get_blacklist_ip_by_id(
    db_client: web::Data<Client>,
    path: web::Path<String>,
) -> impl Responder {
    let collection: Collection<BlacklistedIp> = db_client
        .database("rustkeeper")
        .collection("blacklisted_ips");

    let id_str = path.into_inner();
    let oid = match ObjectId::parse_str(&id_str) {
        // Corrected method here
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID format"),
    };

    let filter = doc! { "_id": oid };
    match collection.find_one(filter, None).await {
        Ok(Some(blacklisted_ip)) => HttpResponse::Ok().json(blacklisted_ip),
        Ok(None) => HttpResponse::NotFound().body("No entry found with the provided ID"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

// Delete a blacklisted IP by ID
pub async fn delete_blacklist_ip_by_id(
    db_client: web::Data<Client>,
    path: web::Path<String>,
) -> impl Responder {
    let collection: Collection<BlacklistedIp> = db_client
        .database("rustkeeper")
        .collection("blacklisted_ips");

    let id_str = path.into_inner();
    let oid = match ObjectId::parse_str(&id_str) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID format"),
    };

    match collection.delete_one(doc! { "_id": oid }, None).await {
        Ok(delete_result) => {
            if delete_result.deleted_count == 1 {
                HttpResponse::Ok().json("Blacklisted IP successfully deleted")
            } else {
                HttpResponse::NotFound().body("No entry found with the provided ID")
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateInputData {
    pub ip_address: String,
    pub status: String,
}

// Update a blacklisted IP by ID
pub async fn edit_blacklist_ip_by_id(
    db_client: web::Data<Client>,
    path: web::Path<String>,
    data: web::Json<UpdateInputData>, // Data for the update
) -> impl Responder {
    let collection: Collection<BlacklistedIp> = db_client
        .database("rustkeeper")
        .collection("blacklisted_ips");

    let id_str = path.into_inner();
    let oid = match ObjectId::parse_str(&id_str) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID format"),
    };

    let update = doc! {
        "$set": {
            "ip_address": &data.ip_address,
            "status": &data.status,
            "updated_at": bson::DateTime::now(),  // Assuming you have an 'updated_at' field to modify
        }
    };

    match collection
        .update_one(doc! { "_id": oid }, update, None)
        .await
    {
        Ok(update_result) => {
            if update_result.modified_count == 1 {
                HttpResponse::Ok().json("Blacklisted IP successfully updated")
            } else {
                HttpResponse::NotFound()
                    .body("No entry found with the provided ID or no changes made")
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
