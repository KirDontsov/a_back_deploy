pub mod config;
pub mod create_avito_ad;
pub mod create_avito_ad_field;
pub mod create_avito_ad_field_value;
pub mod delete_avito_ad;
pub mod delete_avito_ad_field;
pub mod delete_avito_ad_field_value;
pub mod get_all_avito_ads;
pub mod get_avito_ad_by_id;
pub mod get_avito_ad_field_by_id;
pub mod get_avito_ad_field_value_by_id;
pub mod models;
pub mod update_avito_ad;
pub mod update_avito_ad_field;
pub mod update_avito_ad_field_value;

use actix_web::web;

pub fn avito_editor_config(cfg: &mut web::ServiceConfig) {
	cfg.configure(config::avito_editor_routes);
}
