use crate::jwt_auth::JwtMiddleware;
use crate::{
	models::{AvitoRequest, AvitoRequestsDataWithCount, AvitoRequestsResponse},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;

// GET all accaunts avito requests
#[actix_web::get("/avito_requests")]
pub async fn get_all_avito_requests(
	data: web::Data<AppState>,
	user: JwtMiddleware,
) -> Result<HttpResponse> {
	let mut conn = data.db.get().unwrap();

	// Get total count
	let total_count: i64 = crate::schema::avito_requests::table
		.count()
		.get_result(&mut conn)
		.unwrap_or(0);

	match crate::schema::avito_requests::table.load::<AvitoRequest>(&mut conn) {
		Ok(avito_requests) => Ok(HttpResponse::Ok().json(AvitoRequestsResponse {
			status: "success".to_string(),
			data: AvitoRequestsDataWithCount {
				avito_requests,
				count: total_count,
			},
		})),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to fetch avito requests"
		}))),
	}
}
