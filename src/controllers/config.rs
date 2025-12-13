use crate::AppState;
use actix_web::{web, HttpRequest};
use crate::controllers::auth;
use crate::controllers::users;
use crate::controllers::avito_accounts;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .configure(auth::auth_config)
        .configure(users::users_config)
        .configure(avito_accounts::avito_accounts_config);

    conf.service(scope);
}
