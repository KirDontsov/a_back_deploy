pub mod ai_description_processing;
pub mod ai_title_processing;
pub mod config;

pub use self::ai_description_processing::*;
pub use self::ai_title_processing::*;
use actix_web::web;

pub fn avito_client_config(cfg: &mut web::ServiceConfig) {
	cfg.configure(config::avito_ai_processing_routes);
}
