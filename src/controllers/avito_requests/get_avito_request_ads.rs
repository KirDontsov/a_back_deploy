use crate::jwt_auth::JwtMiddleware;
use crate::{
	models::{AvitoAnalyticsAd, FilterOptions},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use diesel::query_dsl::methods::{LimitDsl, OffsetDsl};
use serde_json::json;
use uuid::Uuid;

// GET avito request with ads
#[actix_web::get("/avito_requests/{avito_request_id}/ads")]
pub async fn get_avito_request_ads(
	path: web::Path<Uuid>,
	opts: web::Query<FilterOptions>,
	data: web::Data<AppState>,
	_: JwtMiddleware,
) -> Result<HttpResponse> {
	let avito_request_id = path.into_inner();
	let limit = opts.limit.unwrap_or(10);
	let offset = (opts.page.unwrap_or(1) - 1) * limit;

	let mut conn = data.db.get().unwrap();

	// Get ads by avito_request_id
	let ads_result = LimitDsl::limit(
		OffsetDsl::offset(
			crate::schema::avito_analytics_ads::table
				.filter(crate::schema::avito_analytics_ads::avito_request_id.eq(avito_request_id))
				.order(crate::schema::avito_analytics_ads::position.asc()),
			offset as i64,
		),
		limit as i64,
	)
	.load::<AvitoAnalyticsAd>(&mut conn);

	match ads_result {
		Ok(ads) => {
			// Get total count of ads for this request
			let ads_count = crate::schema::avito_analytics_ads::table
				.filter(crate::schema::avito_analytics_ads::avito_request_id.eq(avito_request_id))
				.count()
				.get_result::<i64>(&mut conn)
				.unwrap_or(0);

			let json_response = json!({
				"status": "success",
				"data": json!({
					"ads": &ads,
					"count": &ads_count
				})
			});
			Ok(HttpResponse::Ok().json(json_response))
		}
		Err(e) => Ok(HttpResponse::InternalServerError()
			.json(json!({"status": "error","message": format!("{:?}", e)}))),
	}
}
