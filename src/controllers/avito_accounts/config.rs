use actix_web::web;
use crate::controllers::avito_accounts::{
    get_all_avito_accounts,
    get_avito_account_by_id,
    create_avito_account,
    update_avito_account,
    delete_avito_account,
};

pub fn avito_account_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_avito_accounts::get_all_avito_accounts)
        .service(get_avito_account_by_id::get_avito_account_by_id)
        .service(create_avito_account::create_avito_account)
        .service(update_avito_account::update_avito_account)
        .service(delete_avito_account::delete_avito_account);
}