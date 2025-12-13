pub mod login;
pub mod register;
pub mod refresh;
pub mod logout;
pub mod role;
pub mod config;

use actix_web::web;

pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.configure(config::auth_routes);
}