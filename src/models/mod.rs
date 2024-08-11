pub mod blacklisted_ip;
pub use blacklisted_ip::BlacklistedIp;

pub mod malicious;
pub use malicious::MaliciousUrl;

pub mod brigatory_users;
pub use brigatory_users::BrigatoryUser;

pub mod rate_limit;
pub use rate_limit::RateLimitEntry; // Add this line to include the rate limit model
