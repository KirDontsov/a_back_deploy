use crate::{
    models::{AvitoFeed, AvitoFeedData, AvitoFeedResponse, UpdateAvitoFeed},
    AppState,
};
use actix_web::{web, HttpResponse, Result};
use chrono::Utc;
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

#[actix_web::patch("/avito/feeds/{id}")]
pub async fn update_avito_feed(
    path: web::Path<Uuid>,
    data: web::Data<AppState>,
    updated_feed: web::Json<UpdateAvitoFeed>,
) -> Result<HttpResponse> {
    let feed_id = path.into_inner();
    let mut conn = data.db.get().unwrap();

    // Set the updated timestamp
    let update_data = UpdateAvitoFeed {
        category: updated_feed.category.clone(),
        updated_ts: Some(Utc::now()),
    };

    let avito_feed = diesel::update(crate::schema::avito_feeds::table.find(feed_id))
        .set(update_data)
        .get_result::<AvitoFeed>(&mut conn);

    match avito_feed {
        Ok(avito_feed) => Ok(HttpResponse::Ok().json(AvitoFeedResponse {
            status: "success".to_string(),
            data: AvitoFeedData { avito_feed },
        })),
        Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to update avito feed"
        }))),
    }
}