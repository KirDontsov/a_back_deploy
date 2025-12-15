pub mod config;
pub mod delete_me;
pub mod get_all_users;
pub mod get_me;
pub mod get_user_by_id;
pub mod update_me;

use actix_web::web;

pub fn users_config(cfg: &mut web::ServiceConfig) {
	cfg.configure(config::user_routes);
}
