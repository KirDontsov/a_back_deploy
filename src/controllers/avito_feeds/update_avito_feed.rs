use crate::{
	models::{AvitoFeed, UpdateAvitoFeed},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use chrono::Utc;
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

#[actix_web::put("/feeds/{feed_id}")]
pub async fn update_avito_feed(
	path: web::Path<Uuid>,
	feed_data: web::Json<UpdateAvitoFeed>,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	let feed_id = path.into_inner();
	let mut conn = data.db.get().unwrap();

	// Prepare update data with current timestamp
	let updated_data = UpdateAvitoFeed {
		name: feed_data.name.clone(),
		description: feed_data.description.clone(),
		feed_type: feed_data.feed_type.clone(),
		is_active: feed_data.is_active,
		updated_ts: Some(Utc::now().naive_utc()),
	};

	match diesel::update(
		crate::schema::avito_feeds::table.filter(crate::schema::avito_feeds::feed_id.eq(feed_id)),
	)
	.set(&updated_data)
	.get_result::<AvitoFeed>(&mut conn)
	{
		Ok(feed) => Ok(HttpResponse::Ok().json(json!({
			"status": "success",
			"data": {
				"avito_feed": feed
			}
		}))),
		Err(diesel::result::Error::NotFound) => Ok(HttpResponse::NotFound().json(json!({
			"status": "fail",
			"message": "Feed with ID not found"
		}))),
		Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to update feed",
			"details": e.to_string()
		}))),
	}
}
