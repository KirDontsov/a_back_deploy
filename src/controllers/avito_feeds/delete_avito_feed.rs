use crate::{models::AvitoFeed, AppState};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

#[actix_web::delete("/avito/feeds/{id}")]
pub async fn delete_avito_feed(
	path: web::Path<Uuid>,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	let feed_id = path.into_inner();
	let mut conn = data.db.get().unwrap();

	let deleted_feed = diesel::delete(crate::schema::avito_feeds::table.find(feed_id))
		.get_result::<AvitoFeed>(&mut conn);

	match deleted_feed {
		Ok(_) => Ok(HttpResponse::Ok().json(json!({
			"status": "success",
			"message": "Avito feed deleted successfully"
		}))),
		Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": "Failed to delete avito feed"
		}))),
	}
}
