use crate::jwt_auth::JwtMiddleware;
use crate::{
	models::{AvitoRequest, PaginationParams, PaginationResponse, ResponseWithPagination},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use diesel::query_dsl::methods::{LimitDsl, OffsetDsl};
use serde_json::json;

// GET all avito requests by specific user_id
#[actix_web::get("/avito_requests")]
pub async fn get_avito_requests_by_user(
	data: web::Data<AppState>,
	pagination: web::Query<PaginationParams>,
	user: JwtMiddleware,
) -> Result<HttpResponse> {
	let mut conn = match data.db.get() {
		Ok(conn) => conn,
		Err(e) => {
			eprintln!("Error getting database connection: {:?}", e);
			return Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to establish database connection"
			})));
		}
	};

	// Get total count for the specific user
	let total_count: i64 = match crate::schema::avito_requests::table
		.filter(crate::schema::avito_requests::user_id.eq(user.user_id))
		.count()
		.get_result(&mut conn)
	{
		Ok(count) => count,
		Err(e) => {
			eprintln!("Error getting count: {:?}", e);
			return Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to fetch avito requests count"
			})));
		}
	};

	let page = pagination.page.unwrap_or(1).max(1);
	let limit = pagination.limit.unwrap_or(10).min(100); // max 100 per page
	let offset = (page - 1) * limit;

	// Calculate pages
	let pages = if limit > 0 {
		((total_count as f64) / (limit as f64)).ceil() as u32
	} else {
		1
	};

	// Get paginated results for the specific user
	let base_query = crate::schema::avito_requests::table
		.filter(crate::schema::avito_requests::user_id.eq(user.user_id));
	let query_with_offset =
		diesel::query_dsl::methods::OffsetDsl::offset(base_query, offset as i64);
	let query_with_limit =
		diesel::query_dsl::methods::LimitDsl::limit(query_with_offset, limit as i64);

	let avito_requests: Vec<AvitoRequest> = match query_with_limit.load::<AvitoRequest>(&mut conn) {
		Ok(requests) => requests,
		Err(e) => {
			eprintln!("Error getting requests: {:?}", e);
			return Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to fetch avito requests"
			})));
		}
	};

	Ok(HttpResponse::Ok().json(ResponseWithPagination {
		status: "success".to_string(),
		data: avito_requests,
		pagination: PaginationResponse {
			page,
			limit,
			total: total_count,
			pages,
		},
	}))
}
