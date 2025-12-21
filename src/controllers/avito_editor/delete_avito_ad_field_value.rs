use crate::jwt_auth::JwtMiddleware;
use crate::{models::AvitoAdFieldValue, AppState};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

#[actix_web::delete("/avito/ad_field_values/{id}")]
pub async fn delete_avito_ad_field_value(
	user: JwtMiddleware,
	path: web::Path<String>,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	let field_value_id = match path.parse::<Uuid>() {
		Ok(id) => id,
		Err(_) => {
			return Ok(HttpResponse::BadRequest().json(json!({
				"status": "fail",
				"message": "Invalid ID format"
			})));
		}
	};

	let mut conn = data.db.get().unwrap();

	// First, get the ad field value to check if it exists
	let avito_ad_field_value = match crate::schema::avito_ad_field_values::table
		.filter(crate::schema::avito_ad_field_values::field_value_id.eq(field_value_id))
		.first::<AvitoAdFieldValue>(&mut conn)
	{
		Ok(field_value) => field_value,
		Err(diesel::result::Error::NotFound) => {
			return Ok(HttpResponse::NotFound().json(json!({
				"status": "fail",
				"message": "Avito ad field value not found"
			})));
		}
		Err(_) => {
			return Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to fetch avito ad field value"
			})));
		}
	};

	// If the field_value has a field_id, check if the user has access to the account that owns the field
	if let Some(field_id) = avito_ad_field_value.field_id {
		// Get the ad field
		let avito_ad_field = match crate::schema::avito_ad_fields::table
			.filter(crate::schema::avito_ad_fields::field_id.eq(field_id))
			.first::<crate::models::AvitoAdField>(&mut conn)
		{
			Ok(field) => field,
			Err(diesel::result::Error::NotFound) => {
				return Ok(HttpResponse::Forbidden().json(json!({
					"status": "fail",
					"message": "You don't have permission to delete this ad field value"
				})));
			}
			Err(_) => {
				return Ok(HttpResponse::InternalServerError().json(json!({
					"status": "error",
					"message": "Failed to fetch ad field information"
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
				"message": "You don't have permission to delete this ad field value"
			})));
		}
	}

	// Delete the ad field value
	let deleted_count =
		diesel::delete(crate::schema::avito_ad_field_values::table.find(field_value_id))
			.execute(&mut conn);

	match deleted_count {
		Ok(count) if count > 0 => Ok(HttpResponse::Ok().json(json!({
			"status": "success",
			"message": "Avito ad field value deleted successfully"
		}))),
		Ok(_) => Ok(HttpResponse::NotFound().json(json!({
			"status": "fail",
			"message": "Avito ad field value not found"
		}))),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to delete avito ad field value"
		}))),
	}
}
