pub mod config;
pub mod create_avito_request;
pub mod delete_avito_request;
pub mod get_all_avito_requests;
pub mod get_avito_request_ads;
pub mod get_avito_request_ads_csv;
pub mod get_avito_request_by_id;
pub mod get_avito_requests_by_user;
pub mod update_avito_request;

use actix_web::web;

pub fn avito_requests_config(cfg: &mut web::ServiceConfig) {
	cfg.configure(config::avito_request_routes);
}
