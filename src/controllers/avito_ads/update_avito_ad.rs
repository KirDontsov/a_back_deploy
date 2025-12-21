use crate::jwt_auth::JwtMiddleware;
use crate::{
	models::{AvitoAd, AvitoAdData, AvitoAdResponse, UpdateAvitoAd},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

#[actix_web::patch("/avito_ads/{id}")]
pub async fn update_avito_ad(
	user: JwtMiddleware,
	path: web::Path<String>,
	body: web::Json<UpdateAvitoAd>,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	let ad_id = match path.parse::<Uuid>() {
		Ok(id) => id,
		Err(_) => {
			return Ok(HttpResponse::BadRequest().json(json!({
				"status": "fail",
				"message": "Invalid ID format"
			})));
		}
	};

	let mut conn = data.db.get().unwrap();

	// First, get the ad to check if it exists
	let avito_ad = match crate::schema::avito_ads::table
		.filter(crate::schema::avito_ads::ad_id.eq(ad_id))
		.first::<AvitoAd>(&mut conn)
	{
	Ok(ad) => ad,
		Err(diesel::result::Error::NotFound) => {
			return Ok(HttpResponse::NotFound().json(json!({
				"status": "fail",
				"message": "Avito ad not found"
			})));
	},
		Err(_) => {
			return Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to fetch avito ad"
			})));
		}
	};

	// Check if the user has access to the account that owns the feed containing this ad
	let feed = match crate::schema::avito_feeds::table
		.filter(crate::schema::avito_feeds::feed_id.eq(avito_ad.feed_id))
		.first::<crate::models::AvitoFeed>(&mut conn)
	{
		Ok(feed) => feed,
		Err(_) => {
			return Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to fetch feed information"
			})));
	}
	};

	// Check if the user has access to the account that owns this feed
	let user_has_access = match crate::schema::avito_accounts::table
		.filter(crate::schema::avito_accounts::account_id.eq(feed.account_id))
		.filter(crate::schema::avito_accounts::user_id.eq(user.user_id))
		.first::<crate::models::AvitoAccount>(&mut conn)
	{
	Ok(_) => true,
		Err(diesel::result::Error::NotFound) => false,
	Err(_) => {
			return Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to verify permissions"
			})));
		}
	};

	if !user_has_access {
		return Ok(HttpResponse::Forbidden().json(json!({
			"status": "fail",
			"message": "You don't have permission to update this ad"
		})));
	}

	// Update the ad
	let updated_avito_ad = diesel::update(crate::schema::avito_ads::table.find(ad_id))
		.set((
			body.status.as_ref().map(|s| crate::schema::avito_ads::status.eq(s)),
			body.avito_ad_id.as_ref().map(|a| crate::schema::avito_ads::avito_ad_id.eq(a)),
			body.parsed_id.as_ref().map(|p| crate::schema::avito_ads::parsed_id.eq(p)),
		))
		.get_result::<AvitoAd>(&mut conn);

	match updated_avito_ad {
		Ok(avito_ad) => Ok(HttpResponse::Ok().json(AvitoAdResponse {
			status: "success".to_string(),
			data: AvitoAdData { avito_ad },
		})),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to update avito ad"
		}))),
	}
}
