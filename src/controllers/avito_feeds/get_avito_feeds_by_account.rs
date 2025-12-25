use crate::{
	jwt_auth::JwtMiddleware,
	models::{AvitoFeed, PaginationParams, PaginationResponse, ResponseWithPagination},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use diesel::query_dsl::methods::{LimitDsl, OffsetDsl};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct AccountIdRequest {
	pub account_id: Uuid,
}

// GET all avito feeds by specific account_id via POST request body
#[actix_web::post("/avito/feeds")]
pub async fn get_avito_feeds_by_account(
	data: web::Data<AppState>,
	body: web::Json<AccountIdRequest>,
	pagination: web::Query<PaginationParams>,
	_: JwtMiddleware,
) -> Result<HttpResponse> {
	let mut conn = data.db.get().unwrap();

	let account_id = body.account_id;

	// Get total count for the specific account
	let total_count: i64 = crate::schema::avito_feeds::table
		.filter(crate::schema::avito_feeds::account_id.eq(account_id))
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

	// Get paginated results for the specific account
	let avito_feeds_result = LimitDsl::limit(
		OffsetDsl::offset(
			crate::schema::avito_feeds::table
				.filter(crate::schema::avito_feeds::account_id.eq(account_id)),
			offset as i64,
		),
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
		Err(e) => {
			log::error!("Failed to fetch avito feeds: {:?}", e);
			Ok(HttpResponse::InternalServerError().json(serde_json::json!({
				"status": "error",
				"message": "Failed to fetch avito feeds"
			})))
		}
	}
}
