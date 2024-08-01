// src/db/seed.rs
use crate::models::BrigatoryUser;
use actix_web::web::Data;
use bcrypt::hash;
use mongodb::{bson::doc, Client, Collection};
use std::env;

pub async fn seed_admin(db_client: Data<Client>) -> Result<(), Box<dyn std::error::Error>> {
    let collection: Collection<BrigatoryUser> = db_client
        .database("rustkeeper")
        .collection("brigatory_users");

    // Retrieve admin credentials from environment variables
    let admin_email = env::var("ADMIN_EMAIL")?;
    let admin_password = env::var("ADMIN_PASSWORD")?;

    // Check if admin user already exists
    let admin_exists = collection
        .find_one(doc! { "email": &admin_email }, None)
        .await?
        .is_some();

    if !admin_exists {
        // Hash the default admin password
        let hashed_password = hash(&admin_password, bcrypt::DEFAULT_COST)?;

        // Create the admin user
        let admin_user = BrigatoryUser {
            _id: None,
            full_name: "Admin".to_string(),
            email: admin_email,
            password: hashed_password,
            status: "approved".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // Insert the admin user into the database
        collection.insert_one(admin_user, None).await?;
    }

    Ok(())
}
