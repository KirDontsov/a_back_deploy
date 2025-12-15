use crate::jwt_auth::JwtMiddleware;
use crate::{
	models::{AvitoAd, AvitoAdData, AvitoAdResponse},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

#[actix_web::get("/avito_ads/{id}")]
pub async fn get_avito_ad_by_id(
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

	// Check if the ad belongs to an account that the user has access to
	match crate::schema::avito_ads::table
		.inner_join(crate::schema::avito_accounts::table)
		.filter(crate::schema::avito_ads::ad_id.eq(ad_id))
		.filter(crate::schema::avito_accounts::user_id.eq(user.user_id))
		.first::<(AvitoAd, crate::models::AvitoAccount)>(&mut conn)
	{
		Ok((avito_ad, _)) => Ok(HttpResponse::Ok().json(AvitoAdResponse {
			status: "success".to_string(),
			data: AvitoAdData { avito_ad },
		})),
		Err(diesel::result::Error::NotFound) => Ok(HttpResponse::NotFound().json(json!({
			"status": "fail",
			"message": "Avito ad not found or you don't have permission to access it"
		}))),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to fetch avito ad"
		}))),
	}
}
