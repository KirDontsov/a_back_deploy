use crate::jwt_auth::JwtMiddleware;
use crate::{models::AvitoAd, AppState};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

use super::models::{AvitoAdFieldWithValues, AvitoAdWithFields, AvitoAdWithFieldsResponse};

#[actix_web::get("/avito_ads/{id}")]
pub async fn get_avito_ad_by_id(
	user: JwtMiddleware,
	path: web::Path<Uuid>,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	let ad_id = path.into_inner();
	let mut conn = data.db.get().unwrap();

	// Get the ad with permission checking
	let avito_ad = match crate::schema::avito_ads::table
		.inner_join(
			crate::schema::avito_feeds::table.inner_join(crate::schema::avito_accounts::table),
		)
		.filter(crate::schema::avito_ads::ad_id.eq(ad_id))
		.filter(crate::schema::avito_accounts::user_id.eq(user.user_id))
		.select(crate::schema::avito_ads::all_columns)
		.first::<AvitoAd>(&mut conn)
	{
		Ok(ad) => ad,
		Err(diesel::result::Error::NotFound) => {
			return Ok(HttpResponse::NotFound().json(json!({
				"status": "fail",
				"message": "Ad not found or you don't have permission to access it"
			})));
		}
		Err(_) => {
			return Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to fetch ad"
			})));
		}
	};

	// Get fields for this ad
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
