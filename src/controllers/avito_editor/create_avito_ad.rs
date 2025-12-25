use crate::jwt_auth::JwtMiddleware;
use crate::{models::AvitoAd, AppState};
use actix_web::{web, HttpResponse, Result};
use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use super::models::{AvitoAdFieldWithValues, AvitoAdWithFields, AvitoAdWithFieldsResponse};
use actix_web::web::Payload;
use futures::StreamExt;
use std::str::FromStr;

#[derive(Deserialize, Serialize)]
struct AvitoAdFieldWithValue {
	tag: Option<String>,
	data_type: Option<String>,
	field_type: Option<String>,
	value: Option<String>,
}

#[actix_web::post("/avito/ads/create")]
pub async fn create_avito_ad(
	user: JwtMiddleware,
	mut payload: Payload,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	// Read the payload into a string
	let mut body = String::new();
	while let Some(chunk) = payload.next().await {
		let chunk = chunk?;
		body.push_str(std::str::from_utf8(&chunk)?);
	}

	// Attempt to parse the JSON
	let parsed: serde_json::Value = match serde_json::from_str(&body) {
		Ok(val) => val,
		Err(_) => {
			return Ok(HttpResponse::BadRequest().json(json!({
				"status": "error",
				"message": "Invalid JSON format"
			})));
		}
	};

	// Extract account_id from the request body
	let account_id = if let Some(account_id_val) = parsed.get("account_id") {
		if let Some(account_id_str) = account_id_val.as_str() {
			match Uuid::parse_str(account_id_str) {
				Ok(uuid) => {
					if uuid.is_nil() {
						return Ok(HttpResponse::BadRequest().json(json!({
							"status": "error",
							"message": "Account ID is required"
						})));
					}
					uuid
				}
				Err(_) => {
					return Ok(HttpResponse::BadRequest().json(json!({
						"status": "error",
						"message": "Invalid Account ID format"
					})));
				}
			}
		} else {
			return Ok(HttpResponse::BadRequest().json(json!({
				"status": "error",
				"message": "Account ID must be a string"
			})));
		}
	} else {
		return Ok(HttpResponse::BadRequest().json(json!({
			"status": "error",
			"message": "Account ID is required in the request body"
		})));
	};

	let mut conn = data.db.get().unwrap();

	// Check if the user has access to the provided account
	let account_exists: Result<crate::models::AvitoAccount, diesel::result::Error> =
		crate::schema::avito_accounts::table
			.filter(crate::schema::avito_accounts::account_id.eq(account_id))
			.filter(crate::schema::avito_accounts::user_id.eq(user.user_id))
			.select(crate::schema::avito_accounts::all_columns)
			.first::<crate::models::AvitoAccount>(&mut conn);

	match account_exists {
		Ok(_) => {} // User has access to this account
		Err(diesel::result::Error::NotFound) => {
			return Ok(HttpResponse::Forbidden().json(json!({
				"status": "fail",
				"message": "You don't have permission to create ads for this account"
			})));
		}
		Err(_) => {
			return Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to verify account access"
			})));
		}
	}

	// Find the newest feed with this account_id and category "MANUAL_CREATE"
	let existing_feed: Option<crate::models::AvitoFeed> = crate::schema::avito_feeds::table
		.filter(crate::schema::avito_feeds::account_id.eq(account_id))
		.filter(crate::schema::avito_feeds::category.eq("MANUAL_CREATE"))
		.order_by(crate::schema::avito_feeds::created_ts.desc())
		.first::<crate::models::AvitoFeed>(&mut conn)
		.optional()
		.map_err(|e| {
			eprintln!("Database error when finding feed: {}", e);
			actix_web::error::ErrorInternalServerError("Database error")
		})?;

	// Determine the feed_id to use - either the existing one or create a new one
	let final_feed_id = match existing_feed {
		Some(feed) => feed.feed_id, // Use existing feed
		None => {
			// Create a new feed for this account
			let new_feed_id = Uuid::new_v4();
			let new_feed = diesel::insert_into(crate::schema::avito_feeds::table)
				.values((
					crate::schema::avito_feeds::feed_id.eq(new_feed_id),
					crate::schema::avito_feeds::account_id.eq(account_id),
					crate::schema::avito_feeds::category.eq("MANUAL_CREATE"),
					crate::schema::avito_feeds::created_ts.eq(chrono::Utc::now().naive_utc()),
				))
				.get_result::<crate::models::AvitoFeed>(&mut conn)
				.map_err(|e| {
					eprintln!("Database error when creating feed: {}", e);
					actix_web::error::ErrorInternalServerError("Failed to create feed")
				})?;

			new_feed.feed_id
		}
	};

	// Extract other fields
	let avito_ad_id = parsed
		.get("avito_ad_id")
		.and_then(|v| v.as_str())
		.map(|s| s.to_string());
	let parsed_id = parsed
		.get("parsed_id")
		.and_then(|v| v.as_str())
		.map(|s| s.to_string());
	let status = parsed
		.get("status")
		.and_then(|v| v.as_str())
		.map(|s| s.to_string());

	// Extract fields - handle both array and object formats
	let fields: Option<Vec<AvitoAdFieldWithValue>> = if let Some(fields_val) = parsed.get("fields")
	{
		if fields_val.is_array() {
			// Handle array format: [{"tag": "name", "value": "value"}, ...]
			let fields_array = fields_val.as_array().unwrap();
			let mut result = Vec::new();
			for field_val in fields_array {
				if let Ok(field) = serde_json::from_value(field_val.clone()) {
					result.push(field);
				}
				// If deserialization of a single field fails, skip it
			}
			Some(result)
		} else if fields_val.is_object() {
			// Handle object format: {"fieldName": "value", ...}
			let mut result = Vec::new();
			if let Some(obj) = fields_val.as_object() {
				for (key, value) in obj {
					let value_str = if value.is_string() {
						value.as_str().map(|s| s.to_string())
					} else {
						Some(value.to_string()) // Convert non-string values to string
					};

					result.push(AvitoAdFieldWithValue {
						tag: Some(key.clone()),
						data_type: None, // Could infer from value type if needed
						field_type: None,
						value: value_str,
					});
				}
			}
			Some(result)
		} else {
			None // If it's neither array nor object, treat as if no fields were provided
		}
	} else {
		None // No fields property provided
	};

	// Now proceed with creating the ad using the final_feed_id
	let new_avito_ad = diesel::insert_into(crate::schema::avito_ads::table)
		.values((
			crate::schema::avito_ads::feed_id.eq(final_feed_id),
			crate::schema::avito_ads::avito_ad_id.eq(&avito_ad_id),
			crate::schema::avito_ads::parsed_id.eq(&parsed_id),
			crate::schema::avito_ads::status.eq(&status),
			crate::schema::avito_ads::created_ts.eq(Utc::now().naive_utc()),
		))
		.get_result::<AvitoAd>(&mut conn);

	match new_avito_ad {
		Ok(avito_ad) => {
			// Now create the associated fields and values if provided
			if let Some(fields) = &fields {
				for field_data in fields {
					// Create the ad field
					let new_field_result =
						diesel::insert_into(crate::schema::avito_ad_fields::table)
							.values((
								crate::schema::avito_ad_fields::ad_id.eq(avito_ad.ad_id),
								crate::schema::avito_ad_fields::tag.eq(&field_data.tag),
								crate::schema::avito_ad_fields::data_type.eq(&field_data.data_type),
								crate::schema::avito_ad_fields::field_type
									.eq(&field_data.field_type),
								crate::schema::avito_ad_fields::created_ts.eq(Utc::now()),
							))
							.get_result::<crate::models::AvitoAdField>(&mut conn);

					if let Ok(new_field) = new_field_result {
						// Create the field value if provided
						if let Some(value) = &field_data.value {
							let _ =
								diesel::insert_into(crate::schema::avito_ad_field_values::table)
									.values((
										crate::schema::avito_ad_field_values::field_id
											.eq(new_field.field_id),
										crate::schema::avito_ad_field_values::value.eq(value),
										crate::schema::avito_ad_field_values::created_ts
											.eq(Utc::now()),
									))
									.get_result::<crate::models::AvitoAdFieldValue>(&mut conn);
						}
					} else {
						// Log the error but continue processing other fields
						eprintln!("Failed to create field: {:?}", new_field_result.err());
					}
				}
			}

			// Get the created ad with its fields and values for the response
			let fields_result = match crate::schema::avito_ad_fields::table
				.filter(crate::schema::avito_ad_fields::ad_id.eq(avito_ad.ad_id))
				.load::<crate::models::AvitoAdField>(&mut conn)
			{
				Ok(fields) => fields,
				Err(_) => Vec::new(), // Continue with empty fields if there's an error
			};

			let mut fields_with_values = Vec::new();
			for field in fields_result {
				// Get values for this field
				let values = match crate::schema::avito_ad_field_values::table
					.filter(crate::schema::avito_ad_field_values::field_id.eq(field.field_id))
					.load::<crate::models::AvitoAdFieldValue>(&mut conn)
				{
					Ok(values) => values,
					Err(_) => Vec::new(), // Continue with empty values if there's an error
				};

				fields_with_values.push(AvitoAdFieldWithValues { field, values });
			}

			let ad_with_fields = AvitoAdWithFields {
				ad: avito_ad,
				fields: fields_with_values,
			};

			Ok(HttpResponse::Ok().json(AvitoAdWithFieldsResponse {
				status: "success".to_string(),
				data: ad_with_fields,
			}))
		}
		Err(diesel::result::Error::DatabaseError(
			diesel::result::DatabaseErrorKind::ForeignKeyViolation,
			_,
		)) => Ok(HttpResponse::BadRequest().json(json!({
			"status": "fail",
			"message": "Feed ID does not exist"
		}))),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to create avito ad"
		}))),
	}
}
