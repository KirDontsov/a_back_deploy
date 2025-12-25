use crate::models::{AvitoAd, AvitoAdField, AvitoAdFieldValue};
use serde::Serialize;

#[derive(Serialize)]
pub struct AvitoAdWithFields {
	pub ad: AvitoAd,
	pub fields: Vec<AvitoAdFieldWithValues>,
}

#[derive(Serialize)]
pub struct AvitoAdFieldWithValues {
	pub field: AvitoAdField,
	pub values: Vec<AvitoAdFieldValue>,
}

#[derive(Serialize)]
pub struct AvitoAdWithFieldsResponse {
	pub status: String,
	pub data: AvitoAdWithFields,
}

#[derive(Serialize)]
pub struct AvitoAdsWithFieldsListResponse {
	pub status: String,
	pub data: AvitoAdsWithFieldsListData,
}

#[derive(Serialize)]
pub struct AvitoAdsWithFieldsListData {
	pub avito_ads_with_fields: Vec<AvitoAdWithFields>,
}
