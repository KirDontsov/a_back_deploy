use crate::{
    jwt_auth::JwtMiddleware,
    models::{AvitoFeed, AvitoFeedResponse, CreateAvitoFeed},
    AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateAvitoFeedRequest {
    pub account_id: Uuid,
    pub category: String,
}

// Create avito feed
#[actix_web::post("/avito/feeds/create")]
pub async fn create_avito_feed(
    data: web::Data<AppState>,
    new_feed: web::Json<CreateAvitoFeedRequest>,
    _: JwtMiddleware,
) -> Result<HttpResponse> {
    let mut conn = data.db.get().unwrap();

    let new_feed_db = CreateAvitoFeed {
        account_id: new_feed.account_id,
        category: new_feed.category.clone(),
    };

    match diesel::insert_into(crate::schema::avito_feeds::table)
        .values(new_feed_db)
        .get_result::<AvitoFeed>(&mut conn)
    {
        Ok(avito_feed) => Ok(HttpResponse::Ok().json(AvitoFeedResponse {
            status: "success".to_string(),
            data: crate::models::AvitoFeedData {
                avito_feed,
            },
        })),
        Err(e) => {
            log::error!("Failed to create avito feed in database: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": "Failed to create avito feed"
            })))
        }
    }
}