use crate::jwt_auth::JwtMiddleware;
use crate::{
	models::{AvitoAdField, CreateAvitoAdField},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use chrono::Utc;
use diesel::prelude::*;
use serde_json::json;

#[actix_web::post("/avito/ad_fields")]
pub async fn create_avito_ad_field(
	user: JwtMiddleware,
	body: web::Json<CreateAvitoAdField>,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	if body.ad_id.is_nil() {
		// Check if ad_id is valid (not zero UUID)
		return Ok(HttpResponse::BadRequest().json(json!({
			"status": "error",
			"message": "Ad ID is required"
		})));
	}

	let mut conn = data.db.get().unwrap();

	// Check if the user has access to the ad (through the feed and account hierarchy)
	// First get the ad
	let avito_ad = match crate::schema::avito_ads::table
		.filter(crate::schema::avito_ads::ad_id.eq(body.ad_id))
		.first::<crate::models::AvitoAd>(&mut conn)
	{
		Ok(ad) => ad,
		Err(diesel::result::Error::NotFound) => {
			return Ok(HttpResponse::NotFound().json(json!({
				"status": "fail",
				"message": "Ad not found"
			})));
		}
		Err(_) => {
			return Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to fetch ad"
			})));
		}
	};

	// Get the feed that owns this ad
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
			"message": "You don't have permission to create fields for this ad"
		})));
	}

	// User has access to this ad, proceed with creating the ad field
	let new_avito_ad_field = diesel::insert_into(crate::schema::avito_ad_fields::table)
		.values((
			crate::schema::avito_ad_fields::ad_id.eq(body.ad_id),
			crate::schema::avito_ad_fields::tag.eq(&body.tag),
			crate::schema::avito_ad_fields::data_type.eq(&body.data_type),
			crate::schema::avito_ad_fields::field_type.eq(&body.field_type),
			crate::schema::avito_ad_fields::created_ts.eq(Utc::now()),
		))
		.get_result::<AvitoAdField>(&mut conn);

	match new_avito_ad_field {
		Ok(avito_ad_field) => Ok(HttpResponse::Ok().json(json!({
			"status": "success",
			"data": {
				"avito_ad_field": avito_ad_field
			}
		}))),
		Err(diesel::result::Error::DatabaseError(
			diesel::result::DatabaseErrorKind::ForeignKeyViolation,
			_,
		)) => Ok(HttpResponse::BadRequest().json(json!({
			"status": "fail",
			"message": "Ad ID does not exist"
		}))),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to create avito ad field"
		}))),
	}
}
