use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AvitoTokenCredentials {
	pub client_id: String,
	pub client_secret: String,
	pub grant_type: String,
}

#[derive(Debug, Deserialize)]
pub struct AvitoTokenParams {
	pub avito_token: String,
}

#[derive(Debug, Deserialize)]
pub struct GetAvitoItemsParams {
	pub avito_token: String,
	pub page: Option<i32>,
	pub per_page: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct GetCategoriesTreeParams {
	pub avito_token: String,
}

#[derive(Debug, Deserialize)]
pub struct AvitoEditorCategoryFieldsParams {
	pub avito_token: String,
	pub avito_slug: String,
}

// Define a temporary struct for car marks since the table may not exist
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AvitoCarMark {
	pub car_mark_id: i32,
	pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct GetItemAnalyticsBody {
	pub avito_token: String,
	pub account_id: String,
	pub date_from: String,
	pub date_to: String,
	pub grouping: String,
	pub limit: i32,
	pub metrics: Vec<String>,
	pub offset: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePriceBody {
	pub avito_token: String,
	pub item_id: String,
	pub price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AvitoGetItemsApiResponse {
	pub meta: serde_json::Value,
	pub resources: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AvitoGetBalanceApiResponse {
	pub balance: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AvitoItemAnalyticsResponse {
	pub result: serde_json::Value,
}

#[derive(Debug)]
pub enum ApiError {
	ReqwestError(String),
	DieselError(diesel::result::Error),
	JsonParseError(String),
	AvitoApiError(u16, String),
	Other(String),
}

impl std::fmt::Display for ApiError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ApiError::ReqwestError(e) => write!(f, "Reqwest error: {}", e),
			ApiError::DieselError(e) => write!(f, "Diesel error: {}", e),
			ApiError::JsonParseError(response) => write!(f, "JSON parse error: {}", response),
			ApiError::AvitoApiError(status, message) => {
				write!(f, "Avito API error {}: {}", status, message)
			}
			ApiError::Other(s) => write!(f, "Other error: {}", s),
		}
	}
}

impl std::error::Error for ApiError {}

impl From<reqwest::Error> for ApiError {
	fn from(error: reqwest::Error) -> Self {
		ApiError::ReqwestError(error.to_string())
	}
}

impl From<serde_json::Error> for ApiError {
	fn from(error: serde_json::Error) -> Self {
		ApiError::JsonParseError(error.to_string())
	}
}

impl From<diesel::result::Error> for ApiError {
	fn from(error: diesel::result::Error) -> Self {
		ApiError::DieselError(error)
	}
}

impl actix_web::ResponseError for ApiError {
	fn error_response(&self) -> actix_web::HttpResponse {
		use actix_web::HttpResponse;
		use serde_json::json;

		match self {
			ApiError::ReqwestError(e) => HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": format!("Request error: {}", e)
			})),
			ApiError::DieselError(e) => HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": format!("Database error: {}", e)
			})),
			ApiError::JsonParseError(response) => HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": format!("JSON parse error: {}", response)
			})),
			ApiError::AvitoApiError(status, message) => HttpResponse::BadRequest().json(json!({
				"status": "error",
				"message": format!("Avito API error {}: {}", status, message)
			})),
			ApiError::Other(message) => HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": message
			})),
		}
	}
}
