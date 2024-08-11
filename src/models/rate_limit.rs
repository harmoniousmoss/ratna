// src/models/rate_limit.rs
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RateLimitEntry {
    #[serde(rename = "_id")]
    pub id: Option<bson::oid::ObjectId>,
    pub ip: String,
    pub request_count: i32,
    pub last_request_time: DateTime, // Use BSON DateTime type here
}
