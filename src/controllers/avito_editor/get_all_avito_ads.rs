use crate::jwt_auth::JwtMiddleware;
use crate::{
	models::AvitoAd,
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

use super::models::{AvitoAdWithFields, AvitoAdFieldWithValues, AvitoAdsWithFieldsListResponse, AvitoAdsWithFieldsListData};

#[actix_web::get("/avito_ads")]
pub async fn get_all_avito_ads(
	user: JwtMiddleware,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	let mut conn = data.db.get().unwrap();

	// Get all feeds that belong to the user's accounts
	let user_feeds = match crate::schema::avito_feeds::table
		.inner_join(crate::schema::avito_accounts::table)
		.filter(crate::schema::avito_accounts::user_id.eq(user.user_id))
		.select(crate::schema::avito_feeds::feed_id)
		.load::<uuid::Uuid>(&mut conn)
	{
	Ok(feeds) => feeds,
		Err(_) => {
			return Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to fetch user feeds"
			})));
		}
	};

	// Get all ads from those feeds
	let avito_ads = match crate::schema::avito_ads::table
		.filter(crate::schema::avito_ads::feed_id.eq_any(&user_feeds))
		.load::<AvitoAd>(&mut conn)
	{
		Ok(ads) => ads,
		Err(_) => {
			return Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to fetch avito ads"
			})));
		}
	};

	// For each ad, get its fields and values
	let mut ads_with_fields = Vec::new();
	for ad in avito_ads {
		// Get fields for this ad
		let fields = match crate::schema::avito_ad_fields::table
			.filter(crate::schema::avito_ad_fields::ad_id.eq(ad.ad_id))
			.load::<crate::models::AvitoAdField>(&mut conn)
		{
			Ok(fields) => fields,
			Err(_) => Vec::new(), // Continue with empty fields if there's an error
		};

		let mut fields_with_values = Vec::new();
		for field in fields {
			// Get values for this field
			let values = match crate::schema::avito_ad_field_values::table
				.filter(crate::schema::avito_ad_field_values::field_id.eq(field.field_id))
				.load::<crate::models::AvitoAdFieldValue>(&mut conn)
			{
				Ok(values) => values,
				Err(_) => Vec::new(), // Continue with empty values if there's an error
			};

			fields_with_values.push(AvitoAdFieldWithValues {
				field,
				values,
			});
		}

		ads_with_fields.push(AvitoAdWithFields {
			ad,
			fields: fields_with_values,
		});
	}

	Ok(HttpResponse::Ok().json(AvitoAdsWithFieldsListResponse {
		status: "success".to_string(),
		results: ads_with_fields.len(),
	data: AvitoAdsWithFieldsListData {
			avito_ads_with_fields: ads_with_fields,
		},
	}))
}