use crate::jwt_auth::JwtMiddleware;
use crate::{models::{User, UserResponse, UserData}, AppState};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;

pub fn users_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_me)
        .service(update_me)
        .service(delete_me)
        .service(get_all_users)
        .service(get_user_by_id);
}

#[actix_web::get("/users/me")]
async fn get_me(user: JwtMiddleware, data: web::Data<AppState>) -> Result<HttpResponse> {
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

#[actix_web::patch("/users/me")]
async fn update_me(
    user: JwtMiddleware,
    body: web::Json<crate::models::UpdateUser>,
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

    // Update user
    let updated_user: User = diesel::update(crate::schema::users::table.find(user.user_id))
        .set((
            crate::schema::users::name.eq(body.name.clone()),
            crate::schema::users::email.eq(body.email.clone().unwrap_or_default()),
            crate::schema::users::role.eq(body.role.clone()),
            crate::schema::users::photo.eq(body.photo.clone()),
            crate::schema::users::updated_at.eq(Some(chrono::Utc::now().naive_utc())),
        ))
        .get_result(&mut conn)
        .expect("Error updating user");

    Ok(HttpResponse::Ok().json(UserResponse {
        status: "success".to_string(),
        data: UserData { user: updated_user },
    }))
}

#[actix_web::delete("/users/me")]
async fn delete_me(user: JwtMiddleware, data: web::Data<AppState>) -> Result<HttpResponse> {
    let mut conn = data.db.get().unwrap();

    match diesel::delete(crate::schema::users::table.find(user.user_id))
        .execute(&mut conn)
    {
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

// Additional user management endpoints
#[actix_web::get("/users")]
async fn get_all_users(data: web::Data<AppState>) -> Result<HttpResponse> {
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

#[actix_web::get("/users/{id}")]
async fn get_user_by_id(path: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> Result<HttpResponse> {
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