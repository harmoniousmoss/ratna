mod auth;
mod db;
mod handlers;
mod middleware;
mod models;
mod routes;

use actix_web::{web, App, HttpServer};
use db::seed::seed_admin;
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;

async fn connect_to_mongo() -> mongodb::error::Result<Client> {
    let db_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in .env");
    let client_options = ClientOptions::parse(&db_uri).await?;
    Client::with_options(client_options)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    let mongo_client = connect_to_mongo()
        .await
        .expect("Failed to connect to MongoDB");

    // Seed the admin user
    if let Err(e) = seed_admin(web::Data::new(mongo_client.clone())).await {
        eprintln!("Failed to seed admin user: {}", e);
        return Ok(()); // Or return an error if seeding failure should stop the server
    }

    println!("Brigatory running on http://{}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(mongo_client.clone()))
            .configure(routes::configure_greet)
            .configure(routes::configure_routes)
    })
    .bind(bind_address)?
    .run()
    .await
}
