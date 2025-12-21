use crate::{
    jwt_auth::JwtMiddleware,
    models::{
        ApiError, AvitoAd, AvitoAdField, AvitoAdFieldValue, FeedQueryParams, FeedResponse,
        FieldResponse, FieldValueResponse, AvitoFeed,
    },
    AppState,
};
use actix_web::{
    get,
    web::{self},
    HttpResponse,
};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::expression_methods::ExpressionMethods;
use diesel::dsl::count;
use diesel::{QueryDsl, SelectableHelper};
use serde::Deserialize;
use uuid::Uuid;

use std::collections::HashMap;

// Path parameter for feed_id
#[derive(Deserialize)]
pub struct FeedIdPath {
    pub feed_id: Uuid,
}

#[get("/avito/feeds/{feed_id}")]
pub async fn get_avito_feed_by_id(
    path: web::Path<FeedIdPath>,
    opts: web::Query<FeedQueryParams>,
    data: web::Data<AppState>,
    _: JwtMiddleware,
) -> Result<HttpResponse, ApiError> {
    let feed_id = path.feed_id;
    let page = opts.page.unwrap_or(1);
    let limit = opts.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let mut conn = data.db.get().unwrap();

    // First, get the feed details
    let feed_row = QueryDsl::filter(
        crate::schema::avito_feeds::table,
        crate::schema::avito_feeds::feed_id.eq(&feed_id)
    )
    .select(AvitoFeed::as_select())
    .first::<AvitoFeed>(&mut conn)
    .optional()
    .map_err(|e| ApiError::DieselError(e))?;

    // If no feed exists, return empty response
    let feed = match feed_row {
        Some(feed) => feed,
        None => {
            return Ok(HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "data": null,
                "pagination": {
                    "page": page,
                    "limit": limit,
                    "total": 0,
                    "pages": 0
                }
            })));
        }
    };

    // Get total count of ads for this feed
    let total_ads = QueryDsl::filter(
        crate::schema::avito_ads::table,
        crate::schema::avito_ads::feed_id.eq(&feed_id)
    )
    .count()
    .get_result::<i64>(&mut conn)
    .map_err(|e| ApiError::DieselError(e))?;

    // Get the paginated list of ads for this feed
    let ad_rows = QueryDsl::offset(
        QueryDsl::limit(
            QueryDsl::order(
                QueryDsl::filter(
                    crate::schema::avito_ads::table,
                    crate::schema::avito_ads::feed_id.eq(&feed_id)
                ),
                crate::schema::avito_ads::created_ts.desc()
            )
            .select(AvitoAd::as_select()),
            limit as i64
        ),
        offset as i64
    )
    .load::<AvitoAd>(&mut conn)
    .map_err(|e| ApiError::DieselError(e))?;

    // Create ads map with empty fields initially and collect ad_ids
    let mut ads_map: HashMap<Uuid, crate::models::AdResponse> = HashMap::new();
    let mut ad_ids: Vec<Uuid> = Vec::new();

    for ad in &ad_rows {
        // Convert NaiveDateTime to DateTime<Utc>
        let created_ts: DateTime<Utc> = ad.created_ts.and_utc();
        
        let ad_response = crate::models::AdResponse {
            ad_id: ad.ad_id,
            avito_ad_id: ad.avito_ad_id.clone().unwrap_or_default(),
            parsed_id: ad.parsed_id.clone().unwrap_or_default(),
            is_active: ad.status.as_ref().map_or(true, |s| s != "inactive"), // Using status field to determine active status
            status: ad.status.clone().unwrap_or_else(|| "unknown".to_string()),
            created_ts,
            fields: Vec::new(),
        };
        ads_map.insert(ad.ad_id, ad_response);
        ad_ids.push(ad.ad_id);
    }

    // Get all fields for these ads only if we have ads
    let mut fields_rows = Vec::new();
    let mut fields_map: HashMap<Uuid, FieldResponse> = HashMap::new();

    if !ad_ids.is_empty() {
        fields_rows = QueryDsl::order(
            QueryDsl::filter(
                crate::schema::avito_ad_fields::table,
                crate::schema::avito_ad_fields::ad_id.eq_any(&ad_ids)
            )
            .select(AvitoAdField::as_select()),
            crate::schema::avito_ad_fields::created_ts.asc()
        )
        .load::<AvitoAdField>(&mut conn)
        .map_err(|e| ApiError::DieselError(e))?;

        // Create fields map with empty values initially
        for field in &fields_rows {
            let field_response = FieldResponse {
                field_id: field.field_id,
                tag: field.tag.clone().unwrap_or_default(),
                data_type: field
                    .data_type
                    .clone()
                    .unwrap_or_else(|| "string".to_string()),
                field_type: field
                    .field_type
                    .clone()
                    .unwrap_or_else(|| "attribute".to_string()),
                created_ts: field.created_ts,
                values: Vec::new(),
            };
            fields_map.insert(field.field_id, field_response);
        }

        // Get all field values for these fields
        let field_ids: Vec<Uuid> = fields_rows.iter().map(|row| row.field_id).collect();

        if !field_ids.is_empty() {
            let field_values_rows = QueryDsl::order(
                QueryDsl::filter(
                    crate::schema::avito_ad_field_values::table,
                    crate::schema::avito_ad_field_values::field_id.eq_any(&field_ids)
                )
                .select(AvitoAdFieldValue::as_select()),
                crate::schema::avito_ad_field_values::created_ts.asc()
            )
            .load::<AvitoAdFieldValue>(&mut conn)
            .map_err(|e| ApiError::DieselError(e))?;

            // Add values to their respective fields
            for row in &field_values_rows {
                if let Some(field_id) = row.field_id {
                    if let Some(field) = fields_map.get_mut(&field_id) {
                        field.values.push(FieldValueResponse {
                            field_value_id: row.field_value_id,
                            value: row.value.clone().unwrap_or_default(),
                            created_ts: row.created_ts,
                        });
                    }
                }
            }
        }
    }

    // Attach fields to their respective ads
    for field in &fields_rows {
        if let Some(ad) = ads_map.get_mut(&field.ad_id) {
            if let Some(field_response) = fields_map.get(&field.field_id) {
                ad.fields.push(field_response.clone());
            }
        }
    }

    // Convert HashMap to Vec maintaining the original order from ad_rows
    let mut ads_vec: Vec<crate::models::AdResponse> = Vec::new();
    for ad in &ad_rows {
        if let Some(ad_response) = ads_map.get(&ad.ad_id) {
            ads_vec.push(ad_response.clone());
        }
    }

    // Create the FeedResponse with the feed data and paginated ads
    let feed_response = FeedResponse {
        feed_id: feed.feed_id,
        account_id: feed.account_id,
        category: feed.category,
        created_ts: feed.created_ts,
        ads: ads_vec,
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": feed_response,
        "pagination": {
            "page": page,
            "limit": limit,
            "total": total_ads as u32,
            "pages": (total_ads as f64 / limit as f64).ceil() as u32
        }
    })))
}