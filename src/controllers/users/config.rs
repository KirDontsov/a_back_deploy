use actix_web::web;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(super::get_me::get_me)
        .service(super::update_me::update_me)
        .service(super::delete_me::delete_me)
        .service(super::get_all_users::get_all_users)
        .service(super::get_user_by_id::get_user_by_id);
}