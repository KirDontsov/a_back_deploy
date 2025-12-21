use crate::controllers::avito_client::{
	get_avito_balance, get_avito_item_analytics, get_avito_items, get_avito_token,
	get_avito_user_profile, get_categories_tree, get_category_fields, update_avito_price,
};
use actix_web::web;

pub fn avito_client_routes(cfg: &mut web::ServiceConfig) {
	cfg.service(get_categories_tree::get_categories_tree)
		.service(get_category_fields::get_avito_category_fields)
		.service(get_avito_token::get_avito_token_handler)
		.service(get_avito_items::get_avito_items)
		.service(get_avito_balance::get_avito_balance)
		.service(get_avito_user_profile::get_avito_user_profile)
		.service(get_avito_item_analytics::get_avito_item_analytics)
		.service(update_avito_price::update_avito_price);
}
