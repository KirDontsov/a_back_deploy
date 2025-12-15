use crate::{
	models::{AvitoFeed, CreateAvitoFeed},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;

#[actix_web::post("/feeds")]
pub async fn create_avito_feed(
	feed_data: web::Json<CreateAvitoFeed>,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	let mut conn = data.db.get().unwrap();

	// Create new feed with current timestamp
	let new_feed = CreateAvitoFeed {
		account_id: feed_data.account_id,
		name: feed_data.name.clone(),
		description: feed_data.description.clone(),
		feed_type: feed_data.feed_type.clone(),
		is_active: feed_data.is_active,
	};

	match diesel::insert_into(crate::schema::avito_feeds::table)
		.values(&new_feed)
		.get_result::<AvitoFeed>(&mut conn)
	{
		Ok(feed) => Ok(HttpResponse::Ok().json(json!({
			"status": "success",
			"data": {
				"avito_feed": feed
			}
		}))),
		Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to create feed",
			"details": e.to_string()
		}))),
	}
}
