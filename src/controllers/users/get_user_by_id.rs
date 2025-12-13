use crate::{models::{User, UserResponse, UserData}, AppState};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;

#[actix_web::get("/users/{id}")]
pub async fn get_user_by_id(path: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> Result<HttpResponse> {
    let mut conn = data.db.get().unwrap();
    let user_id = path.into_inner();

    match crate::schema::users::table.find(user_id).first::<User>(&mut conn) {
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