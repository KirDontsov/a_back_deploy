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
	pub feed_id: Uuid,
	pub avito_ad_id: Option<String>,
	pub parsed_id: Option<String>,
	pub status: Option<String>,
	pub created_ts: NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = avito_ads)]
pub struct CreateAvitoAd {
	pub feed_id: Uuid,
	pub avito_ad_id: Option<String>,
	pub parsed_id: Option<String>,
	pub status: Option<String>,
	pub created_ts: Option<NaiveDateTime>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = avito_ads)]
pub struct UpdateAvitoAd {
	pub status: Option<String>,
	pub avito_ad_id: Option<String>,
	pub parsed_id: Option<String>,
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
pub struct AvitoAdsData {
	pub avito_ads: Vec<AvitoAd>,
}
