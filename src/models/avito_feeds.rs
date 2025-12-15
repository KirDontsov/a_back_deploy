use crate::schema::avito_feeds;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = avito_feeds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AvitoFeed {
	pub feed_id: Uuid,
	pub account_id: Uuid,
	pub name: String,
	pub description: Option<String>,
	pub feed_type: Option<String>,
	pub is_active: Option<bool>,
	pub created_ts: NaiveDateTime,
	pub updated_ts: NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = avito_feeds)]
pub struct CreateAvitoFeed {
	pub account_id: Uuid,
	pub name: String,
	pub description: Option<String>,
	pub feed_type: Option<String>,
	pub is_active: Option<bool>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = avito_feeds)]
pub struct UpdateAvitoFeed {
	pub name: Option<String>,
	pub description: Option<String>,
	pub feed_type: Option<String>,
	pub is_active: Option<bool>,
	pub updated_ts: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub struct AvitoFeedResponse {
	pub status: String,
	pub data: AvitoFeedData,
}

#[derive(Serialize)]
pub struct AvitoFeedData {
	pub avito_feed: AvitoFeed,
}

#[derive(Serialize)]
pub struct AvitoFeedsResponse {
	pub status: String,
	pub results: usize,
	pub data: AvitoFeedsData,
}

#[derive(Serialize)]
pub struct AvitoFeedsData {
	pub avito_feeds: Vec<AvitoFeed>,
}
