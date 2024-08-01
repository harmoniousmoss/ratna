use crate::models::BrigatoryUser;
use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use mongodb::{bson::doc, Client, Collection};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SignupData {
    pub full_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct SigninData {
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

// Handler for user signin
pub async fn signin(db_client: web::Data<Client>, data: web::Json<SigninData>) -> impl Responder {
    let collection: Collection<BrigatoryUser> = db_client
        .database("rustkeeper")
        .collection("brigatory_users");

    // Find the user by email
    let filter = doc! { "email": &data.email };
    let user = match collection.find_one(filter, None).await {
        Ok(user) => user,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };

    // Check if the user exists
    let user = match user {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().body("Invalid email or password"),
    };

    // Check if the user status is not pending
    if user.status == "pending" {
        return HttpResponse::Unauthorized().body("Account is pending approval");
    }

    // Verify the password
    match verify(&data.password, &user.password) {
        Ok(true) => HttpResponse::Ok().json("Signin successful"),
        Ok(false) => HttpResponse::Unauthorized().body("Invalid email or password"),
        Err(_) => HttpResponse::InternalServerError().body("Error verifying password"),
    }
}
