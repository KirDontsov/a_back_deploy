use crate::controllers::avito_ads::{
	create_avito_ad, delete_avito_ad, get_all_avito_ads, get_avito_ad_by_id, update_avito_ad,
};
use actix_web::web;

pub fn avito_ad_routes(cfg: &mut web::ServiceConfig) {
	cfg.service(get_all_avito_ads::get_all_avito_ads)
		.service(get_avito_ad_by_id::get_avito_ad_by_id)
		.service(create_avito_ad::create_avito_ad)
		.service(update_avito_ad::update_avito_ad)
		.service(delete_avito_ad::delete_avito_ad);
}
