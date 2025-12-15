use crate::{
	models::{AuthResponse, RegisterRequest, User},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use bcrypt::DEFAULT_COST;
use diesel::prelude::*;
use diesel::QueryResult;
use serde_json::json;

#[actix_web::post("/auth/register")]
pub async fn register(
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
