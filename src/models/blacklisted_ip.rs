use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlacklistedIp {
    pub ip_address: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status: String, // "blocked" by default, can be changed to "disabled"
}

impl BlacklistedIp {
    pub fn new(ip_address: String) -> Self {
        let now = Utc::now();
        BlacklistedIp {
            ip_address,
            created_at: now,
            updated_at: now,
            status: "blocked".to_string(),
        }
    }
}
