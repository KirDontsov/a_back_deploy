use crate::controllers::avito_editor::{
	create_avito_ad, create_avito_ad_field, create_avito_ad_field_value, delete_avito_ad,
	delete_avito_ad_field, delete_avito_ad_field_value, get_all_avito_ads, get_avito_ad_by_id,
	get_avito_ad_field_by_id, get_avito_ad_field_value_by_id, update_avito_ad, update_avito_ad_field,
	update_avito_ad_field_value,
};
use actix_web::web;

pub fn avito_editor_routes(cfg: &mut web::ServiceConfig) {
	cfg.service(create_avito_ad::create_avito_ad)
		.service(get_all_avito_ads::get_all_avito_ads)
	.service(get_avito_ad_by_id::get_avito_ad_by_id)
		.service(update_avito_ad::update_avito_ad)
	.service(delete_avito_ad::delete_avito_ad)
	.service(create_avito_ad_field::create_avito_ad_field)
		.service(get_avito_ad_field_by_id::get_avito_ad_field_by_id)
		.service(update_avito_ad_field::update_avito_ad_field)
		.service(delete_avito_ad_field::delete_avito_ad_field)
		.service(create_avito_ad_field_value::create_avito_ad_field_value)
		.service(get_avito_ad_field_value_by_id::get_avito_ad_field_value_by_id)
		.service(update_avito_ad_field_value::update_avito_ad_field_value)
	.service(delete_avito_ad_field_value::delete_avito_ad_field_value);
}
