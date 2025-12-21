use crate::jwt_auth::JwtMiddleware;
use crate::{
	models::{AvitoAd, AvitoAdsData, AvitoAdsResponse},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;

#[actix_web::get("/avito_ads")]
pub async fn get_all_avito_ads(
	user: JwtMiddleware,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	let mut conn = data.db.get().unwrap();

	// Get all feeds that belong to the user's accounts
	let user_feeds = match crate::schema::avito_feeds::table
		.inner_join(crate::schema::avito_accounts::table)
		.filter(crate::schema::avito_accounts::user_id.eq(user.user_id))
		.select(crate::schema::avito_feeds::feed_id)
		.load::<uuid::Uuid>(&mut conn)
	{
		Ok(feeds) => feeds,
		Err(_) => {
			return Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to fetch user feeds"
			})));
		}
	};

	// Get all ads from those feeds
	match crate::schema::avito_ads::table
		.filter(crate::schema::avito_ads::feed_id.eq_any(&user_feeds))
		.load::<AvitoAd>(&mut conn)
	{
		Ok(avito_ads) => Ok(HttpResponse::Ok().json(AvitoAdsResponse {
			status: "success".to_string(),
			results: avito_ads.len(),
			data: AvitoAdsData { avito_ads },
		})),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to fetch avito ads"
		}))),
	}
}
