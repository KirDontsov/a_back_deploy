use crate::{
	jwt_auth::JwtMiddleware,
	models::{AvitoFeed, AvitoFeedsDataWithCount, AvitoFeedsResponse},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;

// GET all avito feeds
#[actix_web::get("/avito/feeds")]
pub async fn get_all_avito_feeds(
	data: web::Data<AppState>,
	_: JwtMiddleware,
) -> Result<HttpResponse> {
	let mut conn = data.db.get().unwrap();

	// Get total count
	let total_count: i64 = crate::schema::avito_feeds::table
		.count()
		.get_result(&mut conn)
		.unwrap_or(0);

	match crate::schema::avito_feeds::table.load::<AvitoFeed>(&mut conn) {
		Ok(avito_feeds) => Ok(HttpResponse::Ok().json(AvitoFeedsResponse {
			status: "success".to_string(),
			data: AvitoFeedsDataWithCount {
				avito_feeds,
				count: total_count,
			},
		})),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to fetch avito feeds"
		}))),
	}
}
