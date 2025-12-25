use crate::jwt_auth::JwtMiddleware;
use crate::{models::AvitoAd, AppState};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use super::models::{AvitoAdFieldWithValues, AvitoAdWithFields, AvitoAdWithFieldsResponse};

#[derive(Deserialize)]
struct UpdateAvitoAdWithFields {
	status: Option<String>,
	avito_ad_id: Option<String>,
	parsed_id: Option<String>,
	// Fields and values to be updated along with the ad
	fields_to_update: Option<Vec<AvitoAdFieldToUpdate>>,
	fields_to_create: Option<Vec<AvitoAdFieldToCreate>>,
}

#[derive(Deserialize)]
struct AvitoAdFieldToUpdate {
	field_id: Uuid,
	tag: Option<String>,
	data_type: Option<String>,
	field_type: Option<String>,
	values_to_update: Option<Vec<AvitoAdFieldValueToUpdate>>,
	values_to_create: Option<Vec<AvitoAdFieldValueToCreate>>,
}

#[derive(Deserialize)]
struct AvitoAdFieldToCreate {
	tag: Option<String>,
	data_type: Option<String>,
	field_type: Option<String>,
	values_to_create: Option<Vec<AvitoAdFieldValueToCreate>>,
}

#[derive(Deserialize)]
struct AvitoAdFieldValueToUpdate {
	field_value_id: Uuid,
	value: Option<String>,
}

#[derive(Deserialize)]
struct AvitoAdFieldValueToCreate {
	value: Option<String>,
}

#[actix_web::put("/avito_ads/{id}")]
pub async fn update_avito_ad(
	user: JwtMiddleware,
	path: web::Path<Uuid>,
	body: web::Json<UpdateAvitoAdWithFields>,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	let ad_id = path.into_inner();
	let mut conn = data.db.get().unwrap();

	// Check if the user has access to the ad (through the feed and account hierarchy)
	let ad_exists = match crate::schema::avito_ads::table
		.inner_join(
			crate::schema::avito_feeds::table.inner_join(crate::schema::avito_accounts::table),
		)
		.filter(crate::schema::avito_ads::ad_id.eq(ad_id))
		.filter(crate::schema::avito_accounts::user_id.eq(user.user_id))
		.select(crate::schema::avito_ads::ad_id)
		.first::<Uuid>(&mut conn)
	{
		Ok(id) => id,
		Err(diesel::result::Error::NotFound) => {
			return Ok(HttpResponse::Forbidden().json(json!({
				"status": "fail",
				"message": "You don't have permission to update this ad or it doesn't exist"
			})));
		}
		Err(_) => {
			return Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to verify ad access"
			})));
		}
	};

	// User has access to this ad, proceed with updating the ad itself
	let updated_avito_ad = diesel::update(
		crate::schema::avito_ads::table.filter(crate::schema::avito_ads::ad_id.eq(ad_id)),
	)
	.set((
		crate::schema::avito_ads::status.eq(&body.status),
		crate::schema::avito_ads::avito_ad_id.eq(&body.avito_ad_id),
		crate::schema::avito_ads::parsed_id.eq(&body.parsed_id),
	))
	.get_result::<AvitoAd>(&mut conn);

	match updated_avito_ad {
		Ok(avito_ad) => {
			// Now update the associated fields and values if provided
			if let Some(fields_to_update) = &body.fields_to_update {
				for field_update in fields_to_update {
					// Update the field itself
					let _ = diesel::update(crate::schema::avito_ad_fields::table.filter(
						crate::schema::avito_ad_fields::field_id.eq(field_update.field_id),
					))
					.set((
						crate::schema::avito_ad_fields::tag.eq(&field_update.tag),
						crate::schema::avito_ad_fields::data_type.eq(&field_update.data_type),
						crate::schema::avito_ad_fields::field_type.eq(&field_update.field_type),
					))
					.execute(&mut conn);

					// Update field values if provided
					if let Some(values_to_update) = &field_update.values_to_update {
						for value_update in values_to_update {
							let _ = diesel::update(
								crate::schema::avito_ad_field_values::table.filter(
									crate::schema::avito_ad_field_values::field_value_id
										.eq(value_update.field_value_id),
								),
							)
							.set(
								crate::schema::avito_ad_field_values::value.eq(&value_update.value),
							)
							.execute(&mut conn);
						}
					}

					// Create new field values if provided
					if let Some(values_to_create) = &field_update.values_to_create {
						for value_create in values_to_create {
							let _ =
								diesel::insert_into(crate::schema::avito_ad_field_values::table)
									.values((
										crate::schema::avito_ad_field_values::field_id
											.eq(field_update.field_id),
										crate::schema::avito_ad_field_values::value
											.eq(&value_create.value),
										crate::schema::avito_ad_field_values::created_ts
											.eq(chrono::Utc::now()),
									))
									.get_result::<crate::models::AvitoAdFieldValue>(&mut conn);
						}
					}
				}
			}

			// Create new fields if provided
			if let Some(fields_to_create) = &body.fields_to_create {
				for field_create in fields_to_create {
					// Create the field
					if let Ok(new_field) =
						diesel::insert_into(crate::schema::avito_ad_fields::table)
							.values((
								crate::schema::avito_ad_fields::ad_id.eq(avito_ad.ad_id),
								crate::schema::avito_ad_fields::tag.eq(&field_create.tag),
								crate::schema::avito_ad_fields::data_type
									.eq(&field_create.data_type),
								crate::schema::avito_ad_fields::field_type
									.eq(&field_create.field_type),
								crate::schema::avito_ad_fields::created_ts.eq(chrono::Utc::now()),
							))
							.get_result::<crate::models::AvitoAdField>(&mut conn)
					{
						// Create field values if provided
						if let Some(values_to_create) = &field_create.values_to_create {
							for value_create in values_to_create {
								let _ = diesel::insert_into(
									crate::schema::avito_ad_field_values::table,
								)
								.values((
									crate::schema::avito_ad_field_values::field_id
										.eq(new_field.field_id),
									crate::schema::avito_ad_field_values::value
										.eq(&value_create.value),
									crate::schema::avito_ad_field_values::created_ts
										.eq(chrono::Utc::now()),
								))
								.get_result::<crate::models::AvitoAdFieldValue>(&mut conn);
							}
						}
					}
				}
			}

			// Get updated fields for the response
			let fields = match crate::schema::avito_ad_fields::table
				.filter(crate::schema::avito_ad_fields::ad_id.eq(avito_ad.ad_id))
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
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to update avito ad"
		}))),
	}
}
