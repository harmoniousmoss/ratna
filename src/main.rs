use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Rust Keeper Here")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_address = "127.0.0.1:8080";

    println!("Rust Keeper running on http://{}", bind_address);

    HttpServer::new(|| App::new().route("/", web::get().to(greet)))
        .bind(bind_address)?
        .run()
        .await
}
