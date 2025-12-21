use crate::schema::avito_feeds;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// Generic structure to hold any XML tag and its value
#[derive(Debug)]
pub struct XmlAd {
    pub id: String,
    pub fields: HashMap<String, String>,
}



#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = avito_feeds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AvitoFeed {
    pub feed_id: Uuid,
    pub account_id: Uuid,
    pub category: String,
    pub created_ts: DateTime<Utc>,
    pub updated_ts: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = avito_feeds)]
pub struct CreateAvitoFeed {
    pub account_id: Uuid,
    pub category: String,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = avito_feeds)]
pub struct UpdateAvitoFeed {
    pub category: Option<String>,
    pub updated_ts: Option<DateTime<Utc>>,
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
    pub data: AvitoFeedsDataWithCount,
}

#[derive(Serialize)]
pub struct AvitoFeedsData {
    pub avito_feeds: Vec<AvitoFeed>,
}

#[derive(Serialize)]
pub struct AvitoFeedsDataWithCount {
    #[serde(rename = "avito_feeds")]
    pub avito_feeds: Vec<AvitoFeed>,
    #[serde(rename = "avito_feeds_count")]
    pub count: i64,
}