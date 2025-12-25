use crate::schema::avito_requests;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = avito_requests)]
pub struct AvitoRequest {
	pub request_id: Uuid,
	pub request: String,
	pub city: Option<String>,
	pub coords: Option<String>,
	pub radius: Option<String>,
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
	pub radius: Option<String>,
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
	#[serde(default)]
	pub radius: Option<String>,
	#[serde(default)]
	pub district: Option<String>,
}

// Struct that includes user_id for insertion
#[derive(Insertable)]
#[diesel(table_name = avito_requests)]
pub struct CreateAvitoRequestWithUserId {
	pub request: String,
	pub city: Option<String>,
	pub coords: Option<String>,
	pub radius: Option<String>,
	pub district: Option<String>,
	pub user_id: Uuid,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = avito_requests)]
pub struct UpdateAvitoRequest {
	pub request: Option<String>,
	pub city: Option<String>,
	pub coords: Option<String>,
	pub radius: Option<String>,
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
pub struct AvitoRequestsData {
	pub avito_requests: Vec<AvitoRequest>,
}
