use crate::{models::AvitoRequest, AppState};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

#[actix_web::delete("/avito_requests/{id}")]
pub async fn delete_avito_request(
	path: web::Path<Uuid>,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	let request_id = path.into_inner();
	let mut conn = data.db.get().unwrap();

	let deleted_request = diesel::delete(crate::schema::avito_requests::table.find(request_id))
		.get_result::<AvitoRequest>(&mut conn);

	match deleted_request {
		Ok(_) => Ok(HttpResponse::Ok().json(json!({
			"status": "success",
			"message": "Avito request deleted successfully"
		}))),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to delete avito request"
		}))),
	}
}
