use crate::jwt_auth::JwtMiddleware;
use crate::{
	models::{AvitoRequest, AvitoRequestsResponse, AvitoRequestsDataWithCount},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use diesel::query_dsl::methods::{LimitDsl, OffsetDsl};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationParams {
	page: Option<u32>,
	limit: Option<u32>,
}

// GET all avito requests by specific user_id
#[actix_web::get("/avito_requests")]
pub async fn get_avito_requests_by_user(
	data: web::Data<AppState>,
	pagination: web::Query<PaginationParams>,
	user: JwtMiddleware,
) -> Result<HttpResponse> {
	let mut conn = data.db.get().unwrap();

	let page = pagination.page.unwrap_or(1).max(1);
	let limit = pagination.limit.unwrap_or(10).min(100); // max 100 per page
	let offset = (page - 1) * limit;

	use diesel::query_dsl::methods::{LimitDsl, OffsetDsl};

	// Get total count for the specific user
	let total_count: i64 = crate::schema::avito_requests::table
		.filter(crate::schema::avito_requests::user_id.eq(user.user_id))
		.count()
		.get_result(&mut conn)
		.unwrap_or(0);

	// Get paginated results for the specific user
	let avito_requests: Vec<AvitoRequest> = LimitDsl::limit(
		OffsetDsl::offset(
			crate::schema::avito_requests::table
				.filter(crate::schema::avito_requests::user_id.eq(user.user_id)),
			offset as i64,
		),
		limit as i64,
	)
	.load::<AvitoRequest>(&mut conn)
	.unwrap_or_default();

	Ok(HttpResponse::Ok().json(AvitoRequestsResponse {
		status: "success".to_string(),
		data: AvitoRequestsDataWithCount {
			avito_requests,
			count: total_count,
		},
	}))
}