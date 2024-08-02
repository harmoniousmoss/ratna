use crate::middleware::jwt_auth::JwtAuth;
use actix_web::{web, HttpResponse};

use crate::handlers::{
    add_blacklist_ip, add_blacklist_url, delete_blacklist_ip_by_id, delete_blacklist_url_by_id,
    edit_blacklist_ip_by_id, edit_blacklist_url_by_id, get_all_blacklist_ip, get_all_blacklist_url,
    get_blacklist_ip_by_id, get_blacklist_url_by_id, is_blacklist_url, signin, signup,
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Blacklist IP endpoints
        .service(
            web::resource("/blacklist-ip")
                .route(web::post().to(add_blacklist_ip).wrap(JwtAuth))
                .route(web::get().to(get_all_blacklist_ip)),
        )
        .service(
            web::resource("/blacklist-ip/{id}")
                .route(web::get().to(get_blacklist_ip_by_id))
                .route(web::delete().to(delete_blacklist_ip_by_id).wrap(JwtAuth))
                .route(web::put().to(edit_blacklist_ip_by_id).wrap(JwtAuth)),
        )
        // Blacklist URL endpoints
        .service(
            web::resource("/blacklist-url")
                .route(web::post().to(add_blacklist_url).wrap(JwtAuth))
                .route(web::get().to(get_all_blacklist_url)),
        )
        .service(
            web::resource("/blacklist-url/{id}")
                .route(web::get().to(get_blacklist_url_by_id))
                .route(web::delete().to(delete_blacklist_url_by_id).wrap(JwtAuth))
                .route(web::put().to(edit_blacklist_url_by_id).wrap(JwtAuth)),
        )
        .service(
            web::resource("/check-blacklist") // Add this route
                .route(web::post().to(is_blacklist_url)),
        )
        // User endpoints
        .service(web::resource("/signup").route(web::post().to(signup)))
        .service(web::resource("/signin").route(web::post().to(signin)));
}

pub fn configure_greet(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(|| async { HttpResponse::Ok().body("Brigatory Here") })),
    );
}
