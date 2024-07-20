use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Rust Keeper Here")
}

async fn check_mongo_connection() -> impl Responder {
    match connect_to_mongo().await {
        Ok(_) => HttpResponse::Ok().body("Successfully connected to MongoDB"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn connect_to_mongo() -> mongodb::error::Result<Client> {
    let db_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in .env");
    let client_options = ClientOptions::parse(&db_uri).await?;
    Client::with_options(client_options)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let bind_address = "127.0.0.1:8080";

    println!("Rust Keeper running on http://{}", bind_address);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/check-mongo", web::get().to(check_mongo_connection))
    })
    .bind(bind_address)?
    .run()
    .await
}
