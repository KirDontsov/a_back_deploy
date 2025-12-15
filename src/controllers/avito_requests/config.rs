use crate::controllers::avito_requests::{
	create_avito_request, delete_avito_request, get_all_avito_requests, get_avito_request_ads,
	get_avito_request_ads_csv, get_avito_request_by_id, get_avito_requests_by_user, update_avito_request,
};
use actix_web::web;

pub fn avito_request_routes(cfg: &mut web::ServiceConfig) {
	cfg.service(get_avito_requests_by_user::get_avito_requests_by_user)
		.service(get_avito_request_by_id::get_avito_request_by_id)
		.service(get_avito_request_ads::get_avito_request_ads)
		.service(get_avito_request_ads_csv::get_avito_request_ads_csv)
		.service(get_all_avito_requests::get_all_avito_requests)
	.service(create_avito_request::create_avito_request)
		.service(update_avito_request::update_avito_request)
		.service(delete_avito_request::delete_avito_request);
}
