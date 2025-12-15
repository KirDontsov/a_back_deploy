use crate::jwt_auth::JwtMiddleware;
use crate::AppState;
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;

#[actix_web::delete("/users/me")]
pub async fn delete_me(user: JwtMiddleware, data: web::Data<AppState>) -> Result<HttpResponse> {
	let mut conn = data.db.get().unwrap();

	match diesel::delete(crate::schema::users::table.find(user.user_id)).execute(&mut conn) {
		Ok(_) => Ok(HttpResponse::Ok().json(json!({
			"status": "success",
			"message": "User deleted successfully"
		}))),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to delete user"
		}))),
	}
}
