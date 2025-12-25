use crate::jwt_auth::JwtMiddleware;
use crate::{
	models::{AvitoAnalyticsAd, PaginationParams, PaginationResponse, ResponseWithPagination},
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
	pagination: web::Query<PaginationParams>,
	data: web::Data<AppState>,
	_: JwtMiddleware,
) -> Result<HttpResponse> {
	let avito_request_id = path.into_inner();
	let page = pagination.page.unwrap_or(1).max(1);
	let limit = pagination.limit.unwrap_or(10).min(100); // max 100 per page
	let offset = (page - 1) * limit;

	let mut conn = data.db.get().unwrap();

	// Get total count of ads for this request
	let total_count = crate::schema::avito_analytics_ads::table
		.filter(crate::schema::avito_analytics_ads::avito_request_id.eq(avito_request_id))
		.count()
		.get_result::<i64>(&mut conn)
		.unwrap_or(0);

	// Calculate pages
	let pages = if limit > 0 {
		((total_count as f64) / (limit as f64)).ceil() as u32
	} else {
		1
	};

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
		Ok(ads) => Ok(HttpResponse::Ok().json(ResponseWithPagination {
			status: "success".to_string(),
			data: ads,
			pagination: PaginationResponse {
				page,
				limit,
				total: total_count,
				pages,
			},
		})),
		Err(e) => Ok(HttpResponse::InternalServerError()
			.json(json!({"status": "error","message": format!("{:?}", e)}))),
	}
}
