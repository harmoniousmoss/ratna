mod auth;
mod db;
mod handlers;
mod middleware;
mod models;
mod routes;

use actix_web::{web, App, HttpServer};
use db::seed::seed_admin;
use dotenv::dotenv;
use env_logger::Env;
use mongodb::{options::ClientOptions, Client};
use std::env;

async fn connect_to_mongo() -> mongodb::error::Result<Client> {
    let db_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in .env");
    let client_options = ClientOptions::parse(&db_uri).await?;
    Client::with_options(client_options)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv().ok();

    // Get the port from the environment variable, default to 8080
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("0.0.0.0:{}", port);

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
