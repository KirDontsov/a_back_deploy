use crate::{
	jwt_auth::JwtMiddleware,
	models::{AvitoFeed, PaginationParams, PaginationResponse, ResponseWithPagination},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;

// GET all avito feeds
#[actix_web::get("/avito/feeds")]
pub async fn get_all_avito_feeds(
	data: web::Data<AppState>,
	pagination: web::Query<PaginationParams>,
	_: JwtMiddleware,
) -> Result<HttpResponse> {
	let mut conn = data.db.get().unwrap();

	// Get total count
	let total_count: i64 = crate::schema::avito_feeds::table
		.count()
		.get_result(&mut conn)
		.unwrap_or(0);

	let page = pagination.page.unwrap_or(1).max(1);
	let limit = pagination.limit.unwrap_or(10).min(100); // max 100 per page
	let offset = (page - 1) * limit;

	// Calculate pages
	let pages = if limit > 0 {
		((total_count as f64) / (limit as f64)).ceil() as u32
	} else {
		1
	};

	// Get paginated results
	use diesel::query_dsl::methods::{LimitDsl, OffsetDsl};
	let avito_feeds_result = LimitDsl::limit(
		OffsetDsl::offset(crate::schema::avito_feeds::table, offset as i64),
		limit as i64,
	)
	.load::<AvitoFeed>(&mut conn);

	match avito_feeds_result {
		Ok(avito_feeds) => Ok(HttpResponse::Ok().json(ResponseWithPagination {
			status: "success".to_string(),
			data: avito_feeds,
			pagination: PaginationResponse {
				page,
				limit,
				total: total_count,
				pages,
			},
		})),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to fetch avito feeds"
		}))),
	}
}
