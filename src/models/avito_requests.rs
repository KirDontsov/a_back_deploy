use crate::schema::avito_requests;
use chrono::{NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = avito_requests)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AvitoRequest {
	pub request_id: Uuid,
	pub request: String,
	pub city: Option<String>,
	pub coords: Option<String>,
	pub radius: Option<i32>,
	pub district: Option<String>,
	pub created_ts: NaiveDateTime,
	pub updated_ts: Option<NaiveDateTime>,
	pub user_id: Uuid,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = avito_requests)]
pub struct CreateAvitoRequest {
	pub request: String,
	pub city: Option<String>,
	pub coords: Option<String>,
	pub radius: Option<i32>,
	pub district: Option<String>,
}

// Custom struct for JSON deserialization that handles empty strings
#[derive(Deserialize, Debug)]
pub struct CreateAvitoRequestJson {
	pub request: String,
	#[serde(default)]
	pub city: Option<String>,
	#[serde(default)]
	pub coords: Option<String>,
	#[serde(deserialize_with = "deserialize_optional_number_from_string")]
	#[serde(default)]
	pub radius: Option<i32>,
	#[serde(default)]
	pub district: Option<String>,
}

// Helper function to deserialize numbers from string values (including empty strings)
fn deserialize_optional_number_from_string<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
	D: serde::Deserializer<'de>,
{
	use serde::Deserialize;
	use serde_json::Value;

	let value = Value::deserialize(deserializer)?;
	match value {
		Value::Number(n) => Ok(n.as_i64().map(|n| n as i32)),
		Value::String(s) if s.is_empty() => Ok(None),
		Value::String(s) => s.parse::<i32>().map(Some).map_err(serde::de::Error::custom),
		Value::Null => Ok(None),
		_ => Ok(None),
	}
}

// Struct that includes user_id for insertion
#[derive(Insertable)]
#[diesel(table_name = avito_requests)]
pub struct CreateAvitoRequestWithUserId {
	pub request: String,
	pub city: Option<String>,
	pub coords: Option<String>,
	pub radius: Option<i32>,
	pub district: Option<String>,
	pub user_id: Uuid,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = avito_requests)]
pub struct UpdateAvitoRequest {
	pub request: Option<String>,
	pub city: Option<String>,
	pub coords: Option<String>,
	pub radius: Option<i32>,
	pub district: Option<String>,
	pub updated_ts: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub struct AvitoRequestResponse {
	pub status: String,
	pub data: AvitoRequestData,
}

#[derive(Serialize)]
pub struct AvitoRequestData {
	pub avito_request: AvitoRequest,
}

#[derive(Serialize)]
pub struct AvitoRequestsResponse {
	pub status: String,
	pub data: AvitoRequestsDataWithCount,
}

#[derive(Serialize)]
pub struct AvitoRequestsData {
	pub avito_requests: Vec<AvitoRequest>,
}

#[derive(Serialize)]
pub struct AvitoRequestsDataWithCount {
	#[serde(rename = "avito_requests")]
	pub avito_requests: Vec<AvitoRequest>,
	#[serde(rename = "avito_requests_count")]
	pub count: i64,
}
