pub mod config;
pub mod create_avito_account;
pub mod delete_avito_account;
pub mod get_all_avito_accounts;
pub mod get_avito_account_by_id;
pub mod update_avito_account;

use actix_web::web;

pub fn avito_accounts_config(cfg: &mut web::ServiceConfig) {
	cfg.configure(config::avito_account_routes);
}
