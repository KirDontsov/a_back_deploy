use crate::{
	models::{AvitoRequest, AvitoRequestData, AvitoRequestResponse, UpdateAvitoRequest},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

#[actix_web::patch("/avito_requests/{id}")]
pub async fn update_avito_request(
	path: web::Path<Uuid>,
	data: web::Data<AppState>,
	updated_request: web::Json<UpdateAvitoRequest>,
) -> Result<HttpResponse> {
	let request_id = path.into_inner();
	let mut conn = data.db.get().unwrap();

	let avito_request = diesel::update(crate::schema::avito_requests::table.find(request_id))
		.set(updated_request.into_inner())
		.get_result::<AvitoRequest>(&mut conn);

	match avito_request {
		Ok(avito_request) => Ok(HttpResponse::Ok().json(AvitoRequestResponse {
			status: "success".to_string(),
			data: AvitoRequestData { avito_request },
		})),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to update avito request"
		}))),
	}
}
