use crate::{models::User, AppState};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;

#[actix_web::get("/users")]
pub async fn get_all_users(data: web::Data<AppState>) -> Result<HttpResponse> {
	let mut conn = data.db.get().unwrap();

	match crate::schema::users::table.load::<User>(&mut conn) {
		Ok(users) => Ok(HttpResponse::Ok().json(json!({
			"status": "success",
			"results": users.len(),
			"data": {
				"users": users
			}
		}))),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to fetch users"
		}))),
	}
}
