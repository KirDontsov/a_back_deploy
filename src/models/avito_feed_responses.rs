use crate::models::{AvitoAd, AvitoAdField, AvitoAdFieldValue};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

// Response structures that match the old implementation
#[derive(Debug, Serialize, Clone)]
pub struct FeedResponse {
    pub feed_id: Uuid,
    pub account_id: Uuid,
    pub category: String,
    pub created_ts: DateTime<Utc>,
    pub ads: Vec<AdResponse>,
}

#[derive(Debug, Serialize, Clone)]
pub struct AdResponse {
    pub ad_id: Uuid,
    pub avito_ad_id: String,
    pub parsed_id: String,
    pub is_active: bool,
    pub status: String,
    pub created_ts: DateTime<Utc>,
    pub fields: Vec<FieldResponse>,
}

#[derive(Debug, Serialize, Clone)]
pub struct FieldResponse {
    pub field_id: Uuid,
    pub tag: String,
    pub data_type: String,
    pub field_type: String,
    pub created_ts: DateTime<Utc>,
    pub values: Vec<FieldValueResponse>,
}

#[derive(Debug, Serialize, Clone)]
pub struct FieldValueResponse {
    pub field_value_id: Uuid,
    pub value: String,
    pub created_ts: DateTime<Utc>,
}

// Query parameters for pagination
#[derive(Debug, serde::Deserialize)]
pub struct FeedQueryParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

// Converting from DB models to response models
impl From<AvitoAd> for AdResponse {
    fn from(ad: AvitoAd) -> Self {
        // Convert NaiveDateTime to DateTime<Utc>
        let created_ts = ad.created_ts.and_utc();

        AdResponse {
            ad_id: ad.ad_id,
            avito_ad_id: ad.avito_ad_id.unwrap_or_default(),
            parsed_id: ad.parsed_id.unwrap_or_default(),
            is_active: ad.status.as_ref().map_or(true, |s| s != "inactive"), // Assuming "inactive" means not active
            status: ad.status.unwrap_or_else(|| "unknown".to_string()),
            created_ts,
            fields: Vec::new(),
        }
    }
}

impl From<AvitoAdField> for FieldResponse {
    fn from(field: AvitoAdField) -> Self {
        FieldResponse {
            field_id: field.field_id,
            tag: field.tag.unwrap_or_default(),
            data_type: field.data_type.unwrap_or_else(|| "string".to_string()),
            field_type: field.field_type.unwrap_or_else(|| "attribute".to_string()),
            created_ts: field.created_ts,
            values: Vec::new(),
        }
    }
}

impl From<AvitoAdFieldValue> for FieldValueResponse {
    fn from(value: AvitoAdFieldValue) -> Self {
        FieldValueResponse {
            field_value_id: value.field_value_id,
            value: value.value.unwrap_or_default(),
            created_ts: value.created_ts,
        }
    }
}