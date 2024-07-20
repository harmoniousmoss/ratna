mod handlers;
mod models;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;

use crate::handlers::add_blacklisted_ip;

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Rust Keeper Here")
}

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

    println!("Rust Keeper running on http://{}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(mongo_client.clone()))
            .route("/", web::get().to(greet))
            .route("/blacklist-ip", web::post().to(add_blacklisted_ip))
    })
    .bind(bind_address)?
    .run()
    .await
}
