use crate::jwt_auth::JwtMiddleware;
use crate::{
	models::{AvitoAd, AvitoAdsData, AvitoAdsResponse},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;

#[actix_web::get("/avito_ads")]
pub async fn get_all_avito_ads(
	user: JwtMiddleware,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	let mut conn = data.db.get().unwrap();

	match crate::schema::avito_ads::table
		.inner_join(crate::schema::avito_accounts::table)
		.filter(crate::schema::avito_accounts::user_id.eq(user.user_id))
		.select(crate::schema::avito_ads::all_columns)
		.load::<AvitoAd>(&mut conn)
	{
		Ok(avito_ads) => Ok(HttpResponse::Ok().json(AvitoAdsResponse {
			status: "success".to_string(),
			results: avito_ads.len(),
			data: AvitoAdsData { avito_ads },
		})),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to fetch avito ads"
		}))),
	}
}
