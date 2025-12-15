use crate::jwt_auth::JwtMiddleware;
use crate::models::{ApiError, GetAvitoItemsParams};
use actix_web::{post, web, HttpResponse, Result};
use reqwest::{
	header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE},
	Client,
};
use serde_json::json;
use std::env;

#[post("/avito/get_items")]
pub async fn get_avito_items(
	opts: web::Json<GetAvitoItemsParams>,
	_: JwtMiddleware,
) -> Result<HttpResponse, ApiError> {
	let avito_token = opts.avito_token.clone();
	let page = opts.page.unwrap_or(0);
	let per_page = opts.per_page.unwrap_or(50).min(1000); // Avito API max per_page is 1000

	let url = env::var("AVITO_BASE_URL")
		.map_err(|_| ApiError::Other("AVITO_BASE_URL not set".to_string()))?;

	let mut headers = HeaderMap::new();
	headers.insert(
		CONTENT_TYPE,
		"application/x-www-form-urlencoded".parse().unwrap(),
	);
	headers.insert(
		AUTHORIZATION,
		format!("Bearer {}", avito_token).parse().unwrap(),
	);

	// Build URL with pagination parameters
	let api_url = format!("{}/core/v1/items?page={}&per_page={}", url, page, per_page);

	// Make request
	let response = Client::builder()
		.danger_accept_invalid_certs(true)
		.build()?
		.get(&api_url)
		.headers(headers)
		.send()
		.await?;

	// Check response status
	if !response.status().is_success() {
		let status_code = response.status().as_u16();
		let error_body = response.text().await?;
		return Err(ApiError::AvitoApiError(status_code, error_body));
	}

	// Parse response
	let response_text = response.text().await?;
	let response_data: serde_json::Value = serde_json::from_str(&response_text)
		.map_err(|e| ApiError::JsonParseError(e.to_string()))?;

	Ok(HttpResponse::Ok().json(json!({
		"status": "success",
		"data": {
			"meta": &response_data["meta"],
			"items": &response_data["resources"],
		},
	})))
}
