use crate::controllers::avito_feeds::{
	create_avito_feed, delete_avito_feed, get_all_avito_feeds, get_avito_feed_by_id,
	get_avito_feeds_by_account, import_avito_xml, update_avito_feed,
};
use actix_web::web;

pub fn avito_feed_routes(cfg: &mut web::ServiceConfig) {
	cfg.service(get_avito_feeds_by_account::get_avito_feeds_by_account)
		.service(get_avito_feed_by_id::get_avito_feed_by_id)
		.service(create_avito_feed::create_avito_feed)
		.service(update_avito_feed::update_avito_feed)
		.service(delete_avito_feed::delete_avito_feed)
		.service(get_all_avito_feeds::get_all_avito_feeds)
		.service(import_avito_xml::import_avito_xml);
}
