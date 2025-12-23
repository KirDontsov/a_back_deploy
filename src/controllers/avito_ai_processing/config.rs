use crate::controllers::avito_ai_processing::{
    ai_description_processing, ai_title_processing,
};
use actix_web::web;

pub fn avito_ai_processing_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(ai_title_processing::create_ai_title_processing_handler)
        .service(ai_description_processing::create_ai_description_processing_handler);
}