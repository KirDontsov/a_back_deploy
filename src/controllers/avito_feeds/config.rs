use actix_web::web;

pub fn avito_feed_routes(cfg: &mut web::ServiceConfig) {
	cfg.service(super::get_all_avito_feeds::get_all_avito_feeds)
		.service(super::get_avito_feed_by_id::get_avito_feed_by_id)
		.service(super::create_avito_feed::create_avito_feed)
		.service(super::update_avito_feed::update_avito_feed)
		.service(super::delete_avito_feed::delete_avito_feed);
}
