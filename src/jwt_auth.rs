use core::fmt;
use std::future::{ready, Ready};

use actix_web::error::ErrorUnauthorized;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, web, FromRequest, HttpMessage, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;

use crate::models::TokenClaims;
use crate::AppState;

#[derive(Debug, Serialize)]
struct ErrorResponse {
	status: String,
	message: String,
}

impl fmt::Display for ErrorResponse {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", serde_json::to_string(&self).unwrap())
	}
}

pub struct JwtMiddleware {
	pub user_id: uuid::Uuid,
}

impl FromRequest for JwtMiddleware {
	type Error = ActixWebError;
	type Future = Ready<Result<Self, Self::Error>>;
	fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
		let data = req.app_data::<web::Data<AppState>>().unwrap();

		let token = req
			.cookie("token")
			.map(|c| c.value().to_string())
			.or_else(|| {
				req.headers()
					.get(http::header::AUTHORIZATION)
					.map(|h| h.to_str().unwrap().split_at(7).1.to_string())
			});

		let status = String::from("fail");

		if token.is_none() {
			let json_error = ErrorResponse {
				status,
				message: String::from("You are not logged in, please log in"),
			};
			return ready(Err(ErrorUnauthorized(json_error)));
		}

		let claims = match decode::<TokenClaims>(
			&token.unwrap(),
			&DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
			&Validation::default(),
		) {
			Ok(c) => c.claims,
			Err(_) => {
				let json_error = ErrorResponse {
					status,
					message: String::from("Authentication error"),
				};
				return ready(Err(ErrorUnauthorized(json_error)));
			}
		};

		// Check if token is expired
		let current_timestamp = chrono::Utc::now().timestamp() as usize;
		if claims.exp < current_timestamp {
			let json_error = ErrorResponse {
				status,
				message: String::from("Token has expired"),
			};
			return ready(Err(ErrorUnauthorized(json_error)));
		}

		let user_id = match uuid::Uuid::parse_str(claims.sub.as_str()) {
			Ok(id) => id,
			Err(_) => {
				let json_error = ErrorResponse {
					status,
					message: String::from("Invalid token payload"),
				};
				return ready(Err(ErrorUnauthorized(json_error)));
			}
		};

		req.extensions_mut()
			.insert::<uuid::Uuid>(user_id.to_owned());

		ready(Ok(JwtMiddleware { user_id }))
	}
}

// Function to generate a new token
pub fn generate_token(
	user_id: uuid::Uuid,
	jwt_secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
	let expiration = chrono::Utc::now()
		.checked_add_signed(chrono::Duration::hours(24))
		.unwrap()
		.timestamp() as usize;
	let iat = chrono::Utc::now().timestamp() as usize;

	let claims = TokenClaims {
		sub: user_id.to_string(),
		exp: expiration,
		iat,
	};

	jsonwebtoken::encode(
		&jsonwebtoken::Header::default(),
		&claims,
		&jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_ref()),
	)
}
