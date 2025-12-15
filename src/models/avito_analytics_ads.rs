use crate::schema::avito_analytics_ads;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = avito_analytics_ads)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AvitoAnalyticsAd {
	pub ad_id: Uuid,
	pub my_ad: Option<String>,
	pub run_date: Option<chrono::DateTime<chrono::Utc>>,
	pub city_query: Option<String>,
	pub search_query: Option<String>,
	pub position: Option<i32>,
	pub views: Option<String>,
	pub views_today: Option<String>,
	pub promotion: Option<String>,
	pub delivery: Option<String>,
	pub ad_date: Option<String>,
	pub avito_ad_id: String,
	pub title: Option<String>,
	pub price: Option<String>,
	pub link: Option<String>,
	pub categories: Option<String>,
	pub seller_id: Option<String>,
	pub seller_name: Option<String>,
	pub seller_type: Option<String>,
	pub register_date: Option<String>,
	pub answer_time: Option<String>,
	pub rating: Option<String>,
	pub reviews_count: Option<String>,
	pub ads_count: Option<String>,
	pub closed_ads_count: Option<String>,
	pub photo_count: Option<String>,
	pub address: Option<String>,
	pub description: Option<String>,
	pub avito_request_id: Option<Uuid>,
	pub created_ts: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = avito_analytics_ads)]
pub struct CreateAvitoAnalyticsAd {
	pub my_ad: Option<String>,
	pub run_date: Option<chrono::DateTime<chrono::Utc>>,
	pub city_query: Option<String>,
	pub search_query: Option<String>,
	pub position: Option<i32>,
	pub views: Option<String>,
	pub views_today: Option<String>,
	pub promotion: Option<String>,
	pub delivery: Option<String>,
	pub ad_date: Option<String>,
	pub avito_ad_id: String,
	pub title: Option<String>,
	pub price: Option<String>,
	pub link: Option<String>,
	pub categories: Option<String>,
	pub seller_id: Option<String>,
	pub seller_name: Option<String>,
	pub seller_type: Option<String>,
	pub register_date: Option<String>,
	pub answer_time: Option<String>,
	pub rating: Option<String>,
	pub reviews_count: Option<String>,
	pub ads_count: Option<String>,
	pub closed_ads_count: Option<String>,
	pub photo_count: Option<String>,
	pub address: Option<String>,
	pub description: Option<String>,
	pub avito_request_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct AvitoAnalyticsAdResponse {
	pub status: String,
	pub data: AvitoAnalyticsAdData,
}

#[derive(Serialize)]
pub struct AvitoAnalyticsAdData {
	pub avito_analytics_ad: AvitoAnalyticsAd,
}

#[derive(Serialize)]
pub struct AvitoAnalyticsAdsResponse {
	pub status: String,
	pub data: AvitoAnalyticsAdsData,
}

#[derive(Serialize)]
pub struct AvitoAnalyticsAdsData {
	pub ads: Vec<AvitoAnalyticsAd>,
	pub ads_count: i64,
}

// Pagination parameters for the controller
#[derive(Deserialize)]
pub struct FilterOptions {
	pub page: Option<u32>,
	pub limit: Option<u32>,
}
