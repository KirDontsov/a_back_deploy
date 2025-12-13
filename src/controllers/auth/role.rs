use crate::jwt_auth::JwtMiddleware;
use crate::{models::User, AppState};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;

// Example role management endpoints
#[actix_web::get("/auth/role")]
pub async fn get_role(user: JwtMiddleware, data: web::Data<AppState>) -> Result<HttpResponse> {
    let mut conn = data.db.get().unwrap();

    match crate::schema::users::table
        .find(user.user_id)
        .first::<User>(&mut conn)
    {
        Ok(user) => Ok(HttpResponse::Ok().json(json!({
            "status": "success",
            "data": {
                "role": user.role
            }
        }))),
        Err(_) => Ok(HttpResponse::NotFound().json(json!({
            "status": "fail",
            "message": "User not found"
        }))),
    }
}

#[actix_web::post("/auth/role")]
pub async fn update_role(
    user: JwtMiddleware,
    body: web::Json<serde_json::Value>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let mut conn = data.db.get().unwrap();

    // Check if user exists
    match crate::schema::users::table
        .find(user.user_id)
        .first::<User>(&mut conn)
    {
        Ok(_) => {},
        Err(_) => {
            return Ok(HttpResponse::NotFound().json(json!({
                "status": "fail",
                "message": "User not found"
            })));
        }
    }

    // Extract role from request body
    let new_role = match body.get("role") {
        Some(role_value) => role_value.as_str().unwrap_or("user").to_string(),
        None => {
            return Ok(HttpResponse::BadRequest().json(json!({
                "status": "fail",
                "message": "Role is required"
            })));
        }
    };

    // Update user role
    let updated_user: User = diesel::update(crate::schema::users::table.find(user.user_id))
        .set(crate::schema::users::role.eq(new_role))
        .get_result(&mut conn)
        .expect("Error updating user role");

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": {
            "user": updated_user
        }
    })))
}