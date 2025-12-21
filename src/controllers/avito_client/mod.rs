pub mod config;
pub mod get_avito_balance;
pub mod get_avito_item_analytics;
pub mod get_avito_items;
pub mod get_avito_token;
pub mod get_avito_user_profile;
pub mod get_categories_tree;
pub mod get_category_fields;
pub mod update_avito_price;

use actix_web::web;

pub fn avito_client_config(cfg: &mut web::ServiceConfig) {
	cfg.configure(config::avito_client_routes);
}
