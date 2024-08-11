// src/handlers/check_rate_limit_handler.rs
use actix_web::{web, HttpResponse, Responder};
use bson::{doc, from_document, to_document, DateTime, Document};
use mongodb::{Client, Collection};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::RateLimitEntry;

#[derive(Deserialize)]
pub struct RateLimitCheck {
    pub ip_address: String,
}

pub async fn check_rate_limit(
    db_client: web::Data<Client>,
    req: web::Json<RateLimitCheck>,
) -> impl Responder {
    let collection: Collection<Document> =
        db_client.database("rustkeeper").collection("rate_limits");

    let filter = doc! { "ip": &req.ip_address };
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    match collection.find_one(filter.clone(), None).await {
        Ok(Some(raw_doc)) => {
            println!("Raw document from DB: {:?}", raw_doc);

            match from_document::<RateLimitEntry>(raw_doc) {
                Ok(mut rate_limit_entry) => {
                    let last_request_time =
                        rate_limit_entry.last_request_time.timestamp_millis() / 1000;

                    let elapsed_time = now - last_request_time;

                    if elapsed_time <= 1 {
                        if rate_limit_entry.request_count >= 2 {
                            println!("Rate limit exceeded for IP: {}", req.ip_address);
                            return HttpResponse::TooManyRequests().json("Rate limit exceeded");
                        } else {
                            rate_limit_entry.request_count += 1;
                        }
                    } else {
                        rate_limit_entry.request_count = 1;
                        rate_limit_entry.last_request_time = DateTime::from_millis(now * 1000);
                    }

                    let update_doc = doc! {
                        "$set": {
                            "request_count": rate_limit_entry.request_count,
                            "last_request_time": rate_limit_entry.last_request_time,
                        }
                    };

                    collection
                        .update_one(filter, update_doc, None)
                        .await
                        .unwrap();
                }
                Err(e) => {
                    println!("Deserialization error: {}", e);
                    return HttpResponse::InternalServerError().json("Deserialization error");
                }
            }
        }
        Ok(None) => {
            let new_entry = RateLimitEntry {
                id: None,
                ip: req.ip_address.clone(),
                request_count: 1,
                last_request_time: DateTime::from_millis(now * 1000),
            };

            let new_entry_doc = to_document(&new_entry).unwrap();

            collection.insert_one(new_entry_doc, None).await.unwrap();
        }
        Err(e) => {
            println!("Database error during find_one: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }

    HttpResponse::Ok().json("Request within rate limit")
}
