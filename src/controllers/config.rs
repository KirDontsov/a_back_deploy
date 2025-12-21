use crate::controllers::auth;
use crate::controllers::avito_accounts;
use crate::controllers::avito_ads;
use crate::controllers::avito_client;
use crate::controllers::avito_feeds;
use crate::controllers::avito_requests;
use crate::controllers::users;
use actix_web::web;

pub fn config(conf: &mut web::ServiceConfig) {
	let scope = web::scope("/api")
		.configure(auth::auth_config)
		.configure(users::users_config)
		.configure(avito_accounts::avito_accounts_config)
		.configure(avito_ads::avito_ads_config)
		.configure(avito_feeds::avito_feeds_config)
		.configure(avito_requests::avito_requests_config)
		.configure(avito_client::avito_client_config);

	conf.service(scope);
}
