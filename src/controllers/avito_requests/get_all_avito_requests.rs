use crate::jwt_auth::JwtMiddleware;
use crate::{
	models::{AvitoRequest, PaginationParams, PaginationResponse, ResponseWithPagination},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;

// GET all accounts avito requests (admin only)
#[actix_web::get("/avito_requests/all")]
pub async fn get_all_avito_requests(
	data: web::Data<AppState>,
	user: JwtMiddleware,
	pagination: web::Query<PaginationParams>,
) -> Result<HttpResponse> {
	let mut conn = data.db.get().unwrap();

	// Check if user has admin role by fetching user from database
	let current_user: crate::models::User = match crate::schema::users::table
		.filter(crate::schema::users::id.eq(user.user_id))
		.first(&mut conn)
	{
		Ok(u) => u,
		Err(_) => {
			return Ok(HttpResponse::Unauthorized().json(json!({
				"status": "error",
				"message": "User not found"
			})));
		}
	};

	// Check if user has admin role
	if current_user.role.as_deref() != Some("admin") {
		return Ok(HttpResponse::Forbidden().json(json!({
				"status": "error",
				"message": "Only admin users can access all requests"
		})));
	}

	// Get total count
	let total_count: i64 = crate::schema::avito_requests::table
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
	let avito_requests_result = LimitDsl::limit(
		OffsetDsl::offset(crate::schema::avito_requests::table, offset as i64),
		limit as i64,
	)
	.load::<AvitoRequest>(&mut conn);

	match avito_requests_result {
		Ok(avito_requests) => Ok(HttpResponse::Ok().json(ResponseWithPagination {
			status: "success".to_string(),
			data: avito_requests,
			pagination: PaginationResponse {
				page,
				limit,
				total: total_count,
				pages,
			},
		})),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to fetch avito requests"
		}))),
	}
}
