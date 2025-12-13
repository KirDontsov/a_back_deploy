use crate::jwt_auth::JwtMiddleware;
use crate::{
    models::{LoginRequest, RegisterRequest, User, UserData, AuthResponse},
    AppState,
};
use actix_web::{web, HttpResponse, Result};
use bcrypt::verify;
use bcrypt::DEFAULT_COST;
use diesel::prelude::*;
use diesel::QueryResult;
use serde_json::json;
use actix_web::HttpMessage;

pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(login)
        .service(register)
        .service(refresh_token)
        .service(logout);
}

#[actix_web::post("/auth/login")]
async fn login(
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

#[actix_web::post("/auth/register")]
async fn register(
    body: web::Json<RegisterRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let mut conn = data.db.get().unwrap();

    // Check if user already exists
    let existing_user: QueryResult<User> = crate::schema::users::table
        .filter(crate::schema::users::email.eq(&body.email))
        .first(&mut conn);

    if existing_user.is_ok() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "status": "fail",
            "message": "User with this email already exists"
        })));
    }

    // Verify passwords match
    if body.password != body.password_confirm {
        return Ok(HttpResponse::BadRequest().json(json!({
            "status": "fail",
            "message": "Passwords do not match"
        })));
    }

    // Hash the password
    let hashed_password = match bcrypt::hash(&body.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Error while hashing password"
            })));
        }
    };

    // Create the user
    let new_user = crate::models::CreateUser {
        name: body.name.clone(),
        email: body.email.clone(),
        password: hashed_password,
        role: Some("user".to_string()),
        photo: None,
        verified: Some(false),
        favourite: vec![], // Empty array for favourites
        created_at: Some(chrono::Utc::now().naive_utc()),
        updated_at: Some(chrono::Utc::now().naive_utc()),
    };

    let created_user: User = diesel::insert_into(crate::schema::users::table)
        .values(&new_user)
        .get_result(&mut conn)
        .expect("Error saving new user");

    // Generate JWT token
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp() as usize;
    let iat = chrono::Utc::now().timestamp() as usize;

    let claims = crate::models::TokenClaims {
        sub: created_user.id.to_string(),
        exp: expiration,
        iat,
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    Ok(HttpResponse::Created().json(AuthResponse {
        status: "success".to_string(),
        token,
    }))
}

#[actix_web::post("/auth/refresh")]
async fn refresh_token(user: JwtMiddleware, data: web::Data<AppState>) -> Result<HttpResponse> {
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

#[actix_web::post("/auth/logout")]
async fn logout() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Logged out successfully"
    })))
}

// Helper function to extract user from request
pub fn extract(req: &actix_web::HttpRequest) -> Option<uuid::Uuid> {
    req.extensions_mut().get::<uuid::Uuid>().copied()
}