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
	if body.title.is_empty() {
		return Ok(HttpResponse::BadRequest().json(json!({
			"status": "error",
			"message": "Title is required"
		})));
	}

	let mut conn = data.db.get().unwrap();

	// Check if the user has access to the account
	let account_exists: Result<crate::models::AvitoAccount, diesel::result::Error> =
		crate::schema::avito_accounts::table
			.filter(crate::schema::avito_accounts::account_id.eq(body.account_id))
			.filter(crate::schema::avito_accounts::user_id.eq(user.user_id))
			.first::<crate::models::AvitoAccount>(&mut conn);

	match account_exists {
		Ok(_) => {
			// User has access to this account, proceed with creating the ad
			let new_avito_ad = diesel::insert_into(crate::schema::avito_ads::table)
				.values((
					crate::schema::avito_ads::account_id.eq(body.account_id),
					crate::schema::avito_ads::title.eq(&body.title),
					crate::schema::avito_ads::description.eq(&body.description),
					crate::schema::avito_ads::price.eq(&body.price),
					crate::schema::avito_ads::status.eq(&body.status),
					crate::schema::avito_ads::created_ts.eq(Utc::now().naive_utc()),
					crate::schema::avito_ads::updated_ts.eq(Utc::now().naive_utc()),
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
					"message": "Account ID does not exist"
				}))),
				Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
					"status": "error",
					"message": "Failed to create avito ad"
				}))),
			}
		}
		Err(diesel::result::Error::NotFound) => Ok(HttpResponse::Forbidden().json(json!({
			"status": "fail",
			"message": "You don't have permission to create ads for this account"
		}))),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to verify account access"
		}))),
	}
}
