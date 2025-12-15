use crate::jwt_auth::JwtMiddleware;
use crate::models::{ApiError, GetItemAnalyticsBody};
use actix_web::{post, web, HttpResponse, Result};
use reqwest::{
	header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
	Client,
};
use serde_json::json;
use std::env;

#[post("/avito/get_item_analytics")]
pub async fn get_avito_item_analytics(
	opts: web::Json<GetItemAnalyticsBody>,
	_: JwtMiddleware,
) -> Result<HttpResponse, ApiError> {
	let avito_token = opts.avito_token.clone();
	let account_id = opts.account_id.clone();

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
		"dateFrom": opts.date_from,
		"dateTo": opts.date_to,
		"grouping": opts.grouping,
		"limit": opts.limit,
		"metrics": opts.metrics,
		"offset": opts.offset
	});

	// Build URL
	let api_url = format!("{}/stats/v2/accounts/{}/items", url, account_id);

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

	let analytics_data: serde_json::Value = serde_json::from_str(&response_text)
		.map_err(|e| ApiError::JsonParseError(e.to_string()))?;

	Ok(HttpResponse::Ok().json(json!({
		"status": "success",
		"data": &analytics_data["result"]
	})))
}
