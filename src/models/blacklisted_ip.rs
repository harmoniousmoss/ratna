use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, Serializer}; // Import Serializer

#[derive(Debug, Serialize, Deserialize)]
pub struct BlacklistedIp {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_objectid_as_string"
    )]
    pub _id: Option<ObjectId>, // Use custom serialization
    pub ip_address: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl BlacklistedIp {
    pub fn new(ip_address: String) -> Self {
        let now = Utc::now();
        BlacklistedIp {
            _id: None,
            ip_address,
            status: "blocked".to_string(),
            created_at: now,
            updated_at: now,
        }
    }
}

// Custom serialization function for ObjectId
fn serialize_objectid_as_string<S>(
    value: &Option<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(oid) => serializer.serialize_str(&oid.to_hex()),
        None => serializer.serialize_none(),
    }
}
