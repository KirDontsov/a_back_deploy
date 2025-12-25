use crate::jwt_auth::JwtMiddleware;
use crate::{models::AvitoAd, AppState};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

#[actix_web::delete("/avito_ads/{id}")]
pub async fn delete_avito_ad(
	user: JwtMiddleware,
	path: web::Path<Uuid>,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	let ad_id = path.into_inner();
	let mut conn = data.db.get().unwrap();

	// Check if the user has access to the ad (through the feed and account hierarchy)
	let ad_exists = match crate::schema::avito_ads::table
		.inner_join(
			crate::schema::avito_feeds::table.inner_join(crate::schema::avito_accounts::table),
		)
		.filter(crate::schema::avito_ads::ad_id.eq(ad_id))
		.filter(crate::schema::avito_accounts::user_id.eq(user.user_id))
		.select(crate::schema::avito_ads::ad_id)
		.first::<Uuid>(&mut conn)
	{
		Ok(id) => id,
		Err(diesel::result::Error::NotFound) => {
			return Ok(HttpResponse::Forbidden().json(json!({
				"status": "fail",
				"message": "You don't have permission to delete this ad or it doesn't exist"
			})));
		}
		Err(_) => {
			return Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to verify ad access"
			})));
		}
	};

	// Get field IDs before deleting the fields to use for deleting values
	let field_ids: Vec<uuid::Uuid> = crate::schema::avito_ad_fields::table
		.filter(crate::schema::avito_ad_fields::ad_id.eq(ad_id))
		.select(crate::schema::avito_ad_fields::field_id)
		.load::<uuid::Uuid>(&mut conn)
		.unwrap_or_default();

	// Delete associated field values
	if !field_ids.is_empty() {
		let _ = diesel::delete(
			crate::schema::avito_ad_field_values::table
				.filter(crate::schema::avito_ad_field_values::field_id.eq_any(&field_ids)),
		)
		.execute(&mut conn);
	}

	// Then delete associated fields
	let _ = diesel::delete(
		crate::schema::avito_ad_fields::table
			.filter(crate::schema::avito_ad_fields::ad_id.eq(ad_id)),
	)
	.execute(&mut conn);

	// Finally delete the ad itself
	let deleted_avito_ad = diesel::delete(
		crate::schema::avito_ads::table.filter(crate::schema::avito_ads::ad_id.eq(ad_id)),
	)
	.get_result::<AvitoAd>(&mut conn);

	match deleted_avito_ad {
		Ok(_) => Ok(HttpResponse::Ok().json(json!({
			"status": "success",
			"message": "Ad and associated fields/values deleted successfully"
		}))),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to delete avito ad"
		}))),
	}
}
