use crate::jwt_auth::JwtMiddleware;
use crate::{
    models::{LoginRequest, User, UserData, AuthResponse},
    AppState,
};
use actix_web::{web, HttpResponse, Result};
use bcrypt::verify;
use diesel::prelude::*;
use serde_json::json;

#[actix_web::post("/auth/login")]
pub async fn login(
    body: web::Json<LoginRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let mut conn = data.db.get().unwrap();

    let user: User = match crate::schema::users::table
        .filter(crate::schema::users::email.eq(&body.email))
        .first(&mut conn)
    {
        Ok(user) => user,
        Err(_) => {
            return Ok(HttpResponse::Unauthorized().json(json!({
                "status": "fail",
                "message": "Invalid email or password"
            })));
        }
    };

    match verify(&body.password, &user.password) {
        Ok(valid) => {
            if !valid {
                return Ok(HttpResponse::Unauthorized().json(json!({
                    "status": "fail",
                    "message": "Invalid email or password"
                })));
            }
        }
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Error while verifying password"
            })));
        }
    }

    // Generate JWT token
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp() as usize;
    let iat = chrono::Utc::now().timestamp() as usize;

    let claims = crate::models::TokenClaims {
        sub: user.id.to_string(),
        exp: expiration,
        iat,
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    Ok(HttpResponse::Ok().json(AuthResponse {
        status: "success".to_string(),
        token,
    }))
}