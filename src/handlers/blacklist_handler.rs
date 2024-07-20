use crate::models::BlacklistedIp;
use actix_web::{web, HttpResponse, Responder};
use futures::stream::StreamExt;
use mongodb::{bson, options::FindOptions, Client, Collection};
use serde::Deserialize;

// Define a helper struct for deserializing incoming data
#[derive(Debug, Deserialize)]
pub struct InputData {
    pub ip_address: String,
}

// Post request handler to add a new IP to the blacklist
pub async fn add_blacklisted_ip(
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
pub async fn get_all_blocked_ips(db_client: web::Data<Client>) -> impl Responder {
    let collection: Collection<BlacklistedIp> = db_client
        .database("your_database_name")
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
