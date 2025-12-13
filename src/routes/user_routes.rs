use actix_web::web;
use crate::handlers::user_handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .route("", web::get().to(user_handlers::get_users))
            .route("", web::post().to(user_handlers::create_user))
            .route("/{id}", web::get().to(user_handlers::get_user))
            .route("/{id}", web::patch().to(user_handlers::update_user))
            .route("/{id}", web::delete().to(user_handlers::delete_user))
    );
}