use crate::{
	models::{AvitoFeed},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;

#[actix_web::get("/feeds")]
pub async fn get_all_avito_feeds(data: web::Data<AppState>) -> Result<HttpResponse> {
	let mut conn = data.db.get().unwrap();

	match crate::schema::avito_feeds::table.load::<AvitoFeed>(&mut conn) {
		Ok(feeds) => Ok(HttpResponse::Ok().json(json!({
			"status": "success",
			"results": feeds.len(),
			"data": {
				"avito_feeds": feeds
			}
		}))),
		Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to fetch feeds",
			"details": e.to_string()
		}))),
	}
}
