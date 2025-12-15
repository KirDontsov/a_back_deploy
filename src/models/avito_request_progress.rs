use crate::schema::avito_request_progress;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = avito_request_progress)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AvitoRequestProgress {
	pub progress_id: Uuid,
	pub request_id: Uuid,
	pub progress: f64,
	pub status: String,
	pub message: String,
	pub total_ads: i32,
	pub current_ads: i32,
	pub created_ts: NaiveDateTime,
	pub updated_ts: NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = avito_request_progress)]
pub struct CreateAvitoRequestProgress {
	pub request_id: Uuid,
	pub progress: f64,
	pub status: String,
	pub message: String,
	pub total_ads: i32,
	pub current_ads: i32,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = avito_request_progress)]
pub struct UpdateAvitoRequestProgress {
	pub progress: Option<f64>,
	pub status: Option<String>,
	pub message: Option<String>,
	pub total_ads: Option<i32>,
	pub current_ads: Option<i32>,
	pub updated_ts: Option<NaiveDateTime>,
}
