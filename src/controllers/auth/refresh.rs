use crate::jwt_auth::JwtMiddleware;
use crate::{models::AuthResponse, AppState};
use actix_web::{web, HttpResponse, Result};
use serde_json::json;

#[actix_web::post("/auth/refresh")]
pub async fn refresh_token(user: JwtMiddleware, data: web::Data<AppState>) -> Result<HttpResponse> {
	// In a real application, you would validate the refresh token here
	// For now, we'll just return a new access token

	let token = match crate::jwt_auth::generate_token(user.user_id, &data.env.jwt_secret) {
		Ok(token) => token,
		Err(_) => {
			return Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to generate new token"
			})));
		}
	};

	Ok(HttpResponse::Ok().json(AuthResponse {
		status: "success".to_string(),
		token,
	}))
}
