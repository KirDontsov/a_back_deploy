use crate::AppState;
use actix_web::{web, HttpRequest};
use crate::controllers::auth;
use crate::controllers::user;
use crate::controllers::users_public;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .configure(auth::auth_config)
        .configure(user::user_config)
        .configure(users_public::users_public_config);

    conf.service(scope);
}
