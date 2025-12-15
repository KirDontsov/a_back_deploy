use crate::{
	models::{AvitoFeed},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

#[actix_web::get("/feeds/{feed_id}")]
pub async fn get_avito_feed_by_id(
	path: web::Path<Uuid>,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	let feed_id = path.into_inner();
	let mut conn = data.db.get().unwrap();

	match crate::schema::avito_feeds::table
		.filter(crate::schema::avito_feeds::feed_id.eq(feed_id))
		.first::<AvitoFeed>(&mut conn)
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
			"message": "Failed to fetch feed",
			"details": e.to_string()
		}))),
	}
}
