pub mod config;
pub mod create_avito_ad;
pub mod delete_avito_ad;
pub mod get_all_avito_ads;
pub mod get_avito_ad_by_id;
pub mod update_avito_ad;

use actix_web::web;

pub fn avito_ads_config(cfg: &mut web::ServiceConfig) {
	cfg.configure(config::avito_ad_routes);
}
