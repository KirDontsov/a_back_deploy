use crate::schema::avito_ad_field_values;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = avito_ad_field_values)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AvitoAdFieldValue {
    pub field_value_id: Uuid,
    pub field_id: Option<Uuid>,
    pub value: Option<String>,
    pub created_ts: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = avito_ad_field_values)]
pub struct CreateAvitoAdFieldValue {
    pub field_id: Option<Uuid>,
    pub value: Option<String>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = avito_ad_field_values)]
pub struct UpdateAvitoAdFieldValue {
    pub value: Option<String>,
}