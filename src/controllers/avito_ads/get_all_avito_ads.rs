use crate::jwt_auth::JwtMiddleware;
use crate::{
	models::{AvitoAd, PaginationParams, PaginationResponse, ResponseWithPagination},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;

#[actix_web::get("/avito_ads")]
pub async fn get_all_avito_ads(
	user: JwtMiddleware,
	pagination: web::Query<PaginationParams>,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	let mut conn = data.db.get().unwrap();

	// Get all feeds that belong to the user's accounts
	let user_feeds = match crate::schema::avito_feeds::table
		.inner_join(crate::schema::avito_accounts::table)
		.filter(crate::schema::avito_accounts::user_id.eq(user.user_id))
		.select(crate::schema::avito_feeds::feed_id)
		.load::<uuid::Uuid>(&mut conn)
	{
		Ok(feeds) => feeds,
		Err(_) => {
			return Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to fetch user feeds"
			})));
		}
	};

	// Get total count for pagination
	let total_count: i64 = crate::schema::avito_ads::table
		.filter(crate::schema::avito_ads::feed_id.eq_any(&user_feeds))
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

	// Get paginated ads from those feeds
	use diesel::query_dsl::methods::{LimitDsl, OffsetDsl};
	let avito_ads_result = LimitDsl::limit(
		OffsetDsl::offset(
			crate::schema::avito_ads::table
				.filter(crate::schema::avito_ads::feed_id.eq_any(&user_feeds)),
			offset as i64,
		),
		limit as i64,
	)
	.load::<AvitoAd>(&mut conn);

	match avito_ads_result {
		Ok(avito_ads) => Ok(HttpResponse::Ok().json(ResponseWithPagination {
			status: "success".to_string(),
			data: avito_ads,
			pagination: PaginationResponse {
				page,
				limit,
				total: total_count,
				pages,
			},
		})),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to fetch avito ads"
		}))),
	}
}
