use crate::models::BrigatoryUser;
use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use mongodb::{bson::doc, Client, Collection};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SignupData {
    pub full_name: String,
    pub email: String,
    pub password: String,
}

// Handler for user signup
pub async fn signup(db_client: web::Data<Client>, data: web::Json<SignupData>) -> impl Responder {
    let collection: Collection<BrigatoryUser> = db_client
        .database("rustkeeper")
        .collection("brigatory_users");

    // Hash the password
    let hashed_password = match hash(&data.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => return HttpResponse::InternalServerError().body("Error hashing password"),
    };

    // Create a new BrigatoryUser instance
    let new_user = BrigatoryUser::new(data.full_name.clone(), data.email.clone(), hashed_password);

    // Insert the new user into the database
    match collection.insert_one(new_user, None).await {
        Ok(_) => HttpResponse::Created().json("User successfully registered with status 'pending'"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
