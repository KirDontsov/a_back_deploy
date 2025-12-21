use crate::schema::avito_ad_fields;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = avito_ad_fields)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AvitoAdField {
    pub field_id: Uuid,
    pub ad_id: Uuid,
    pub tag: Option<String>,
    pub data_type: Option<String>,
    pub field_type: Option<String>,
    pub created_ts: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = avito_ad_fields)]
pub struct CreateAvitoAdField {
    pub ad_id: Uuid,
    pub tag: Option<String>,
    pub data_type: Option<String>,
    pub field_type: Option<String>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = avito_ad_fields)]
pub struct UpdateAvitoAdField {
    pub tag: Option<String>,
    pub data_type: Option<String>,
    pub field_type: Option<String>,
}