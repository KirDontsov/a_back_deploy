use actix_web::web;

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
	cfg.service(super::login::login)
		.service(super::register::register)
		.service(super::refresh::refresh_token)
		.service(super::logout::logout)
		.service(super::role::get_role)
		.service(super::role::update_role);
}
