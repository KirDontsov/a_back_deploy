use crate::jwt_auth::JwtMiddleware;
use crate::models::{ApiError, UpdatePriceBody};
use actix_web::{post, web, HttpResponse, Result};
use reqwest::{
	header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
	Client,
};
use serde_json::json;
use std::env;

#[post("/avito/update_price")]
pub async fn update_avito_price(
	opts: web::Json<UpdatePriceBody>,
	_: JwtMiddleware,
) -> Result<HttpResponse, ApiError> {
	let avito_token = opts.avito_token.clone();
	let item_id = opts.item_id.clone();

	let url = env::var("AVITO_BASE_URL")
		.map_err(|_| ApiError::Other("AVITO_BASE_URL not set".to_string()))?;

	// Build headers
	let mut headers = HeaderMap::new();
	headers.insert(
		AUTHORIZATION,
		format!("Bearer {}", avito_token).parse().unwrap(),
	);
	headers.insert(USER_AGENT, HeaderValue::from_static("YourApp/1.0"));
	headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
	headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

	// Build request body
	let request_body = json!({
		"price": opts.price
	});

	// Build URL
	let api_url = format!("{}/core/v1/items/{}/update_price", url, item_id);

	// Make request
	let response = Client::builder()
		.danger_accept_invalid_certs(true)
		.build()?
		.post(&api_url)
		.headers(headers)
		.json(&request_body)
		.send()
		.await?;

	// Check response status
	if !response.status().is_success() {
		let status_code = response.status().as_u16();
		let error_body = response.text().await?;
		return Err(ApiError::AvitoApiError(status_code, error_body));
	}

	// Parse response
	let response_text: String = response.text().await?;

	let update_price_data: serde_json::Value = serde_json::from_str(&response_text)
		.map_err(|e| ApiError::JsonParseError(e.to_string()))?;

	Ok(HttpResponse::Ok().json(json!({
		"status": "success",
		"data": &update_price_data["result"]
	})))
}
