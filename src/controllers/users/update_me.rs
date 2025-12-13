use crate::jwt_auth::JwtMiddleware;
use crate::{models::UpdateUser, AppState};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;

#[actix_web::patch("/users/me")]
pub async fn update_me(
    user: JwtMiddleware,
    body: web::Json<UpdateUser>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let mut conn = data.db.get().unwrap();

    // Check if user exists
    match crate::schema::users::table
        .find(user.user_id)
        .first::<crate::models::User>(&mut conn)
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
    let updated_user: crate::models::User = diesel::update(crate::schema::users::table.find(user.user_id))
        .set((
            crate::schema::users::name.eq(body.name.clone()),
            crate::schema::users::email.eq(body.email.clone().unwrap_or_default()),
            crate::schema::users::role.eq(body.role.clone()),
            crate::schema::users::photo.eq(body.photo.clone()),
            crate::schema::users::updated_at.eq(Some(chrono::Utc::now().naive_utc())),
        ))
        .get_result(&mut conn)
        .expect("Error updating user");

    Ok(HttpResponse::Ok().json(crate::models::UserResponse {
        status: "success".to_string(),
        data: crate::models::UserData { user: updated_user },
    }))
}