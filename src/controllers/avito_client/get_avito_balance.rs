use crate::jwt_auth::JwtMiddleware;
use crate::models::{ApiError, AvitoTokenParams};
use actix_web::{post, web, HttpResponse, Result};
use reqwest::{
	header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
	Client,
};
use serde_json::json;
use std::env;

#[post("/avito/get_balance")]
pub async fn get_avito_balance(
	opts: web::Json<AvitoTokenParams>,
	_: JwtMiddleware,
) -> Result<HttpResponse, ApiError> {
	let avito_token = opts.avito_token.clone();

	let url = env::var("AVITO_BASE_URL").expect("AVITO_BASE_URL not set");

	let headers: HeaderMap<HeaderValue> = HeaderMap::from_iter(vec![
		(CONTENT_TYPE, "application/json".parse().unwrap()),
		(
			AUTHORIZATION,
			format!("Bearer {}", avito_token).parse().unwrap(),
		),
	]);

	let body = json!({});

	let response = Client::builder()
		.danger_accept_invalid_certs(true)
		.build()?
		.post(format!("{}/cpa/v3/balanceInfo", url))
		.headers(headers)
		.json(&body)
		.send()
		.await?;

	// Check response status
	if !response.status().is_success() {
		let status_code = response.status().as_u16();
		let error_body = response.text().await?;
		return Err(ApiError::AvitoApiError(status_code, error_body));
	}

	let response_text: String = response.text().await?;

	// Parse the response
	let response_data: serde_json::Value = serde_json::from_str(&response_text)
		.map_err(|e| ApiError::JsonParseError(e.to_string()))?;

	Ok(HttpResponse::Ok().json(json!({
		"status": "success",
		"data": {
			"balance": &response_data["balance"],
		}
	})))
}
