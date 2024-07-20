mod handlers;
mod models;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;

use crate::handlers::{
    add_blacklist_ip, delete_blacklist_ip_by_id, get_all_blacklist_ip, get_blacklist_ip_by_id,
};

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
            .route("/blacklist-ip", web::post().to(add_blacklist_ip))
            .route("/blacklist-ip", web::get().to(get_all_blacklist_ip))
            .route("/blacklist-ip/{id}", web::get().to(get_blacklist_ip_by_id))
            .route(
                "/blacklist-ip/{id}",
                web::delete().to(delete_blacklist_ip_by_id),
            )
    })
    .bind(bind_address)?
    .run()
    .await
}
