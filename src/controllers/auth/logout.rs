use actix_web::{HttpResponse, Result};
use serde_json::json;

#[actix_web::post("/auth/logout")]
pub async fn logout() -> Result<HttpResponse> {
	Ok(HttpResponse::Ok().json(json!({
		"status": "success",
		"message": "Logged out successfully"
	})))
}
