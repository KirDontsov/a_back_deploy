use crate::jwt_auth::JwtMiddleware;
use crate::{
	models::{User, UserData, UserResponse},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;

#[actix_web::get("/users/me")]
pub async fn get_me(user: JwtMiddleware, data: web::Data<AppState>) -> Result<HttpResponse> {
	let mut conn = data.db.get().unwrap();

	match crate::schema::users::table
		.find(user.user_id)
		.first::<User>(&mut conn)
	{
		Ok(user) => Ok(HttpResponse::Ok().json(UserResponse {
			status: "success".to_string(),
			data: UserData { user },
		})),
		Err(_) => Ok(HttpResponse::NotFound().json(json!({
			"status": "fail",
			"message": "User not found"
		}))),
	}
}
