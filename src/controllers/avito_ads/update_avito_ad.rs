use crate::jwt_auth::JwtMiddleware;
use crate::{
	models::{AvitoAd, AvitoAdData, AvitoAdResponse, UpdateAvitoAd},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use chrono::Utc;
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

	// Check if the ad exists and belongs to an account that the user has access to
	let existing_ad: Result<(AvitoAd, crate::models::AvitoAccount), diesel::result::Error> =
		crate::schema::avito_ads::table
			.inner_join(crate::schema::avito_accounts::table)
			.filter(crate::schema::avito_ads::ad_id.eq(ad_id))
			.filter(crate::schema::avito_accounts::user_id.eq(user.user_id))
			.first::<(AvitoAd, crate::models::AvitoAccount)>(&mut conn);

	match existing_ad {
		Ok((_, _)) => {
			// Update the ad
			let updated_avito_ad = diesel::update(crate::schema::avito_ads::table.find(ad_id))
				.set((
					body.title
						.as_ref()
						.map(|t| crate::schema::avito_ads::title.eq(t)),
					body.description
						.as_ref()
						.map(|d| crate::schema::avito_ads::description.eq(d)),
					body.price.map(|p| crate::schema::avito_ads::price.eq(p)),
					body.status
						.as_ref()
						.map(|s| crate::schema::avito_ads::status.eq(s)),
					crate::schema::avito_ads::updated_ts.eq(Utc::now().naive_utc()),
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
		Err(diesel::result::Error::NotFound) => Ok(HttpResponse::NotFound().json(json!({
			"status": "fail",
			"message": "Avito ad not found or you don't have permission to update it"
		}))),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to fetch avito ad"
		}))),
	}
}
