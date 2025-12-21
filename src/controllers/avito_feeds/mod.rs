pub mod config;
pub mod create_avito_feed;
pub mod delete_avito_feed;
pub mod get_all_avito_feeds;
pub mod get_avito_feed_by_id;
pub mod get_avito_feeds_by_account;
pub mod import_avito_xml;
pub mod update_avito_feed;

use actix_web::web;

pub fn avito_feeds_config(cfg: &mut web::ServiceConfig) {
	cfg.configure(config::avito_feed_routes);
}
