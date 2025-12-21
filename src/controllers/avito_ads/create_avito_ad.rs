use crate::jwt_auth::JwtMiddleware;
use crate::{
	models::{AvitoAd, AvitoAdData, AvitoAdResponse, CreateAvitoAd},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use chrono::Utc;
use diesel::prelude::*;
use serde_json::json;

#[actix_web::post("/avito_ads")]
pub async fn create_avito_ad(
	user: JwtMiddleware,
	body: web::Json<CreateAvitoAd>,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	if body.feed_id.is_nil() { // Check if feed_id is valid (not zero UUID)
		return Ok(HttpResponse::BadRequest().json(json!({
			"status": "error",
			"message": "Feed ID is required"
		})));
	}

	let mut conn = data.db.get().unwrap();

	// Check if the user has access to the feed (which is linked to an account)
	let feed_exists: Result<crate::models::AvitoFeed, diesel::result::Error> =
		crate::schema::avito_feeds::table
			.inner_join(crate::schema::avito_accounts::table)
			.filter(crate::schema::avito_feeds::feed_id.eq(body.feed_id))
			.filter(crate::schema::avito_accounts::user_id.eq(user.user_id))
			.select(crate::schema::avito_feeds::all_columns)
			.first::<crate::models::AvitoFeed>(&mut conn);

	match feed_exists {
		Ok(_) => {
			// User has access to this feed, proceed with creating the ad
			let new_avito_ad = diesel::insert_into(crate::schema::avito_ads::table)
				.values((
					crate::schema::avito_ads::feed_id.eq(body.feed_id),
					crate::schema::avito_ads::avito_ad_id.eq(&body.avito_ad_id),
					crate::schema::avito_ads::parsed_id.eq(&body.parsed_id),
					crate::schema::avito_ads::status.eq(&body.status),
					crate::schema::avito_ads::created_ts.eq(Utc::now().naive_utc()),
				))
				.get_result::<AvitoAd>(&mut conn);

			match new_avito_ad {
				Ok(avito_ad) => Ok(HttpResponse::Ok().json(AvitoAdResponse {
					status: "success".to_string(),
					data: AvitoAdData { avito_ad },
				})),
				Err(diesel::result::Error::DatabaseError(
					diesel::result::DatabaseErrorKind::ForeignKeyViolation,
					_,
				)) => Ok(HttpResponse::BadRequest().json(json!({
					"status": "fail",
					"message": "Feed ID does not exist"
				}))),
				Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
					"status": "error",
					"message": "Failed to create avito ad"
				}))),
			}
		}
		Err(diesel::result::Error::NotFound) => Ok(HttpResponse::Forbidden().json(json!({
			"status": "fail",
			"message": "You don't have permission to create ads for this feed"
		}))),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to verify feed access"
		}))),
	}
}
