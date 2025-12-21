/// Avito API endpoint to get the categories tree
///
/// Expected request body:
/// {
///     "avito_token": "your_avito_token_here"
/// }
///
/// Response:
/// {
///     "status": "success",
///     "data": {
///         "categories": [...]
///     }
/// }
use crate::jwt_auth::JwtMiddleware;
use crate::models::{ApiError, GetCategoriesTreeParams};
use actix_web::{post, web, HttpResponse, Result};
use reqwest::{
	header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE},
	Client,
};
use serde_json::json;
use std::env;

#[post("/avito/get_categories_tree")]
pub async fn get_categories_tree(
	opts: web::Json<GetCategoriesTreeParams>,
	_: JwtMiddleware,
) -> Result<HttpResponse, ApiError> {
	let avito_token = opts.avito_token.clone();
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

	// Build URL for categories tree
	let api_url = format!("{}/autoload/v1/user-docs/tree", url);

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
		"data": response_data
	})))
}
