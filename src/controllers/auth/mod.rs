pub mod config;
pub mod login;
pub mod logout;
pub mod refresh;
pub mod register;
pub mod role;

use actix_web::web;

pub fn auth_config(cfg: &mut web::ServiceConfig) {
	cfg.configure(config::auth_routes);
}
