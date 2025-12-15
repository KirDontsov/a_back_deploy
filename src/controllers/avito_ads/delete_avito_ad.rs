use crate::jwt_auth::JwtMiddleware;
use crate::{models::AvitoAd, AppState};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

#[actix_web::delete("/avito_ads/{id}")]
pub async fn delete_avito_ad(
	user: JwtMiddleware,
	path: web::Path<String>,
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

	// Check if the ad exists and belongs to an account that the user has access to
	let existing_ad: Result<(AvitoAd, crate::models::AvitoAccount), diesel::result::Error> =
		crate::schema::avito_ads::table
			.inner_join(crate::schema::avito_accounts::table)
			.filter(crate::schema::avito_ads::ad_id.eq(ad_id))
			.filter(crate::schema::avito_accounts::user_id.eq(user.user_id))
			.first::<(AvitoAd, crate::models::AvitoAccount)>(&mut conn);

	match existing_ad {
		Ok(_) => {
			// Delete the ad
			let result =
				diesel::delete(crate::schema::avito_ads::table.find(ad_id)).execute(&mut conn);

			match result {
				Ok(_) => Ok(HttpResponse::Ok().json(json!({
					"status": "success",
					"message": "Avito ad deleted successfully"
				}))),
				Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
					"status": "error",
					"message": "Failed to delete avito ad"
				}))),
			}
		}
		Err(diesel::result::Error::NotFound) => Ok(HttpResponse::NotFound().json(json!({
			"status": "fail",
			"message": "Avito ad not found or you don't have permission to delete it"
		}))),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to fetch avito ad"
		}))),
	}
}
