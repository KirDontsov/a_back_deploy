use crate::jwt_auth::JwtMiddleware;
use crate::{
	models::{AvitoRequest, AvitoRequestData, AvitoRequestResponse},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

// Get my account avito requests
#[actix_web::get("/avito_requests/{id}")]
pub async fn get_avito_request_by_id(
	path: web::Path<Uuid>,
	data: web::Data<AppState>,
	user: JwtMiddleware,
) -> Result<HttpResponse> {
	let request_id = path.into_inner();
	let mut conn = data.db.get().unwrap();

	match crate::schema::avito_requests::table
		.filter(crate::schema::avito_requests::request_id.eq(request_id))
		.filter(crate::schema::avito_requests::user_id.eq(user.user_id))
		.first::<AvitoRequest>(&mut conn)
	{
		Ok(avito_request) => Ok(HttpResponse::Ok().json(AvitoRequestResponse {
			status: "success".to_string(),
			data: AvitoRequestData { avito_request },
		})),
		Err(_) => Ok(HttpResponse::NotFound().json(json!({
			"status": "fail",
			"message": "Avito request not found or does not belong to user"
		}))),
	}
}
