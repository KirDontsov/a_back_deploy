use crate::jwt_auth::JwtMiddleware;
use crate::{
	models::{AvitoAdFieldValue, CreateAvitoAdFieldValue},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use chrono::Utc;
use diesel::prelude::*;
use serde_json::json;

#[actix_web::post("/avito/ad_field_values")]
pub async fn create_avito_ad_field_value(
	user: JwtMiddleware,
	body: web::Json<CreateAvitoAdFieldValue>,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	let mut conn = data.db.get().unwrap();

	// If field_id is provided, check if the user has access to the field through the ad hierarchy
	if let Some(field_id) = body.field_id {
		if field_id.is_nil() {
			return Ok(HttpResponse::BadRequest().json(json!({
				"status": "error",
				"message": "Field ID is invalid"
			})));
		}

		// Check if the user has access to the field (through the ad and account hierarchy)
		let avito_ad_field = match crate::schema::avito_ad_fields::table
			.filter(crate::schema::avito_ad_fields::field_id.eq(field_id))
			.first::<crate::models::AvitoAdField>(&mut conn)
		{
			Ok(field) => field,
			Err(diesel::result::Error::NotFound) => {
				return Ok(HttpResponse::NotFound().json(json!({
					"status": "fail",
					"message": "Avito ad field not found"
				})));
			}
			Err(_) => {
				return Ok(HttpResponse::InternalServerError().json(json!({
					"status": "error",
					"message": "Failed to fetch avito ad field"
				})));
			}
		};

		// Get the ad that owns this field
		let ad = match crate::schema::avito_ads::table
			.filter(crate::schema::avito_ads::ad_id.eq(avito_ad_field.ad_id))
			.first::<crate::models::AvitoAd>(&mut conn)
		{
			Ok(ad) => ad,
			Err(_) => {
				return Ok(HttpResponse::InternalServerError().json(json!({
					"status": "error",
					"message": "Failed to fetch ad information"
				})));
			}
		};

		// Get the feed that owns this ad
		let feed = match crate::schema::avito_feeds::table
			.filter(crate::schema::avito_feeds::feed_id.eq(ad.feed_id))
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
				"message": "You don't have permission to create field values for this field"
			})));
		}

		// User has access to this field, proceed with creating the field value
		let new_avito_ad_field_value =
			diesel::insert_into(crate::schema::avito_ad_field_values::table)
				.values((
					crate::schema::avito_ad_field_values::field_id.eq(&body.field_id),
					crate::schema::avito_ad_field_values::value.eq(&body.value),
					crate::schema::avito_ad_field_values::created_ts.eq(Utc::now()),
				))
				.get_result::<AvitoAdFieldValue>(&mut conn);

		match new_avito_ad_field_value {
			Ok(avito_ad_field_value) => Ok(HttpResponse::Ok().json(json!({
				"status": "success",
				"data": {
					"avito_ad_field_value": avito_ad_field_value
				}
			}))),
			Err(diesel::result::Error::DatabaseError(
				diesel::result::DatabaseErrorKind::ForeignKeyViolation,
				_,
			)) => Ok(HttpResponse::BadRequest().json(json!({
				"status": "fail",
				"message": "Field ID does not exist"
			}))),
			Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to create avito ad field value"
			}))),
		}
	} else {
		// If no field_id is provided, create the field value without permission check
		let new_avito_ad_field_value =
			diesel::insert_into(crate::schema::avito_ad_field_values::table)
				.values((
					crate::schema::avito_ad_field_values::field_id.eq(&body.field_id),
					crate::schema::avito_ad_field_values::value.eq(&body.value),
					crate::schema::avito_ad_field_values::created_ts.eq(Utc::now()),
				))
				.get_result::<AvitoAdFieldValue>(&mut conn);

		match new_avito_ad_field_value {
			Ok(avito_ad_field_value) => Ok(HttpResponse::Ok().json(json!({
				"status": "success",
				"data": {
					"avito_ad_field_value": avito_ad_field_value
				}
			}))),
			Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to create avito ad field value"
			}))),
		}
	}
}
