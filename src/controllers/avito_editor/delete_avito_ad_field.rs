use crate::jwt_auth::JwtMiddleware;
use crate::{models::AvitoAdField, AppState};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

#[actix_web::delete("/avito/ad_fields/{id}")]
pub async fn delete_avito_ad_field(
	user: JwtMiddleware,
	path: web::Path<String>,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	let field_id = match path.parse::<Uuid>() {
		Ok(id) => id,
		Err(_) => {
			return Ok(HttpResponse::BadRequest().json(json!({
				"status": "fail",
				"message": "Invalid ID format"
			})));
		}
	};

	let mut conn = data.db.get().unwrap();

	// First, get the ad field to check if it exists
	let avito_ad_field = match crate::schema::avito_ad_fields::table
		.filter(crate::schema::avito_ad_fields::field_id.eq(field_id))
		.first::<AvitoAdField>(&mut conn)
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

	// Check if the user has access to the account that owns the ad field
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
			"message": "You don't have permission to delete this ad field"
		})));
	}

	// Delete the ad field
	let deleted_count =
		diesel::delete(crate::schema::avito_ad_fields::table.find(field_id)).execute(&mut conn);

	match deleted_count {
		Ok(count) if count > 0 => Ok(HttpResponse::Ok().json(json!({
			"status": "success",
			"message": "Avito ad field deleted successfully"
		}))),
		Ok(_) => Ok(HttpResponse::NotFound().json(json!({
			"status": "fail",
			"message": "Avito ad field not found"
		}))),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to delete avito ad field"
		}))),
	}
}
