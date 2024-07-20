use crate::models::BlacklistedIp;
use actix_web::{web, HttpResponse, Responder};
use mongodb::{Client, Collection};
use serde::Deserialize;

// Define a helper struct for deserializing incoming data
#[derive(Debug, Deserialize)]
pub struct InputData {
    pub ip_address: String,
}

pub async fn add_blacklisted_ip(
    db_client: web::Data<Client>,
    data: web::Json<InputData>, // Change to use InputData
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
