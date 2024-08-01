mod db;
mod handlers;
mod models;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use db::seed::seed_admin;
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;

use crate::handlers::{
    add_blacklist_ip, add_blacklist_url, delete_blacklist_ip_by_id, delete_blacklist_url_by_id,
    edit_blacklist_ip_by_id, edit_blacklist_url_by_id, get_all_blacklist_ip, get_all_blacklist_url,
    get_blacklist_ip_by_id, get_blacklist_url_by_id, signin, signup,
};

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Brigatory Here")
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

    // Seed the admin user
    if let Err(e) = seed_admin(web::Data::new(mongo_client.clone())).await {
        eprintln!("Failed to seed admin user: {}", e);
        return Ok(()); // Or return an error if seeding failure should stop the server
    }

    println!("Brigatory running on http://{}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(mongo_client.clone()))
            .route("/", web::get().to(greet))
            // Blacklist url endpoint
            .route("/blacklist-ip", web::post().to(add_blacklist_ip))
            .route("/blacklist-ip", web::get().to(get_all_blacklist_ip))
            .route("/blacklist-ip/{id}", web::get().to(get_blacklist_ip_by_id))
            .route(
                "/blacklist-ip/{id}",
                web::delete().to(delete_blacklist_ip_by_id),
            )
            .route("/blacklist-ip/{id}", web::put().to(edit_blacklist_ip_by_id))
            // Malicious url endpoint
            .route("/blacklist-url", web::post().to(add_blacklist_url))
            .route("/blacklist-url", web::get().to(get_all_blacklist_url))
            .route(
                "/blacklist-url/{id}",
                web::get().to(get_blacklist_url_by_id),
            )
            .route(
                "/blacklist-url/{id}",
                web::delete().to(delete_blacklist_url_by_id),
            )
            .route(
                "/blacklist-url/{id}",
                web::put().to(edit_blacklist_url_by_id),
            )
            // Users endpoint
            .route("/signup", web::post().to(signup))
            .route("/signin", web::post().to(signin))
    })
    .bind(bind_address)?
    .run()
    .await
}
