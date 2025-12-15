use crate::schema::avito_ads;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = avito_ads)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AvitoAd {
	pub ad_id: Uuid,
	pub account_id: Uuid,
	pub title: String,
	pub description: Option<String>,
	pub price: Option<i32>,
	pub status: Option<String>,
	pub created_ts: NaiveDateTime,
	pub updated_ts: NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = avito_ads)]
pub struct CreateAvitoAd {
	pub account_id: Uuid,
	pub title: String,
	pub description: Option<String>,
	pub price: Option<i32>,
	pub status: Option<String>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = avito_ads)]
pub struct UpdateAvitoAd {
	pub title: Option<String>,
	pub description: Option<String>,
	pub price: Option<i32>,
	pub status: Option<String>,
	pub updated_ts: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub struct AvitoAdResponse {
	pub status: String,
	pub data: AvitoAdData,
}

#[derive(Serialize)]
pub struct AvitoAdData {
	pub avito_ad: AvitoAd,
}

#[derive(Serialize)]
pub struct AvitoAdsResponse {
	pub status: String,
	pub results: usize,
	pub data: AvitoAdsData,
}

#[derive(Serialize)]
pub struct AvitoAdsData {
	pub avito_ads: Vec<AvitoAd>,
}
