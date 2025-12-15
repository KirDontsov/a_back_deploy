use crate::jwt_auth::JwtMiddleware;
use crate::models::{ApiError, AvitoTokenCredentials};
use actix_web::{
	cookie::{time::Duration as ActixWebDuration, Cookie, SameSite},
	post,
	web::{self},
	HttpResponse,
};
use reqwest::{header::CONTENT_TYPE, Client};
use serde_json::json;
use std::env;

#[post("/avito/get_token")]
pub async fn get_avito_token_handler(
	credentials: web::Json<AvitoTokenCredentials>,
	_: JwtMiddleware,
) -> Result<HttpResponse, ApiError> {
	let client_id = credentials.client_id.clone();
	let client_secret = credentials.client_secret.clone();
	let grant_type = credentials.grant_type.clone();

	// Prepare form data for token request
	let params = [
		("client_id", &client_id),
		("client_secret", &client_secret),
		("grant_type", &grant_type),
	];

	let client = Client::new();
	let token_url = format!(
		"{}/token",
		env::var("AVITO_BASE_URL").expect("AVITO_BASE_URL not set")
	);

	let response = client
		.post(&token_url)
		.header(CONTENT_TYPE, "application/x-www-form-urlencoded")
		.form(&params)
		.send()
		.await?;

	if !response.status().is_success() {
		let status_code = response.status().as_u16();
		let error_body = response.text().await?;
		return Err(ApiError::AvitoApiError(status_code, error_body));
	}

	let response_text = response.text().await?;
	let token_response: serde_json::Value = serde_json::from_str(&response_text)
		.map_err(|e| ApiError::JsonParseError(e.to_string()))?;

	let access_token = token_response["access_token"]
		.as_str()
		.ok_or_else(|| ApiError::Other("Access token not found in response".to_string()))?;

	// Build cookie
	let cookie = Cookie::build("avito_token", access_token)
		.same_site(SameSite::None)
		.path("/")
		.max_age(ActixWebDuration::new(3600, 0)) // Default expiration of 1 hour
		.secure(true)
		.finish();

	Ok(HttpResponse::Ok().cookie(cookie).json(json!({
		"status": "success",
		"data": {
			"access_token": access_token,
			"token_type": "Bearer",
			"expires_in": 3600,
		}
	})))
}
