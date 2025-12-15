use crate::jwt_auth::JwtMiddleware;
use crate::utils::transliterate::Translit;
use crate::{models::AvitoAnalyticsAd, AppState};
use actix_web::{web, HttpResponse, Result};
use csv::Writer;
use diesel::prelude::*;
use std::io::Cursor;
use uuid::Uuid;

// GET avito request with ads in a csv file
#[actix_web::get("/avito_requests/{avito_request_id}/ads/csv")]
pub async fn get_avito_request_ads_csv(
	path: web::Path<Uuid>,
	data: web::Data<AppState>,
	_: JwtMiddleware,
) -> Result<HttpResponse> {
	let avito_request_id = path.into_inner();

	let mut conn = data.db.get().unwrap();

	// Get all ads by avito_request_id (no pagination)
	let ads_result = crate::schema::avito_analytics_ads::table
		.filter(crate::schema::avito_analytics_ads::avito_request_id.eq(avito_request_id))
		.order(crate::schema::avito_analytics_ads::position.asc())
		.load::<AvitoAnalyticsAd>(&mut conn);

	match ads_result {
		Ok(ads) => {
			// Create CSV in memory
			let mut writer = Writer::from_writer(Cursor::new(Vec::new()));

			// Write headers
			writer
				.write_record(&[
					"Мое",
					"Дата прогона",
					"Город (запрос)",
					"Поиск (запрос)",
					"Поз.",
					"Просмотров",
					"Просмотров сегодня",
					"Продвижение",
					"Доставка",
					"Дата объявления",
					"id",
					"Название",
					"Цена",
					"Ссылка",
					"Категории",
					"id Продавца",
					"Продавец",
					"Тип продавца",
					"Дата регистрации",
					"Время ответа",
					"Рейтинг",
					"Кол. отзывов",
					"Кол. объявлений",
					"Кол. закрытых",
					"Фото",
					"Адрес",
					"Описание",
				])
				.map_err(|e| {
					actix_web::error::ErrorInternalServerError(format!("CSV write error: {:?}", e))
				})?;

			// Write records
			for ad in &ads {
				writer
					.write_record(&[
						ad.my_ad.as_deref().unwrap_or(""),
						ad.run_date
							.as_ref()
							.map(|dt| dt.to_rfc3339())
							.unwrap_or_else(|| "".to_string())
							.as_str(),
						ad.city_query.as_deref().unwrap_or(""),
						ad.search_query.as_deref().unwrap_or(""),
						ad.position
							.map(|p| p.to_string())
							.unwrap_or(String::new())
							.as_str(),
						ad.views.as_deref().unwrap_or(""),
						ad.views_today.as_deref().unwrap_or(""),
						ad.promotion.as_deref().unwrap_or(""),
						ad.delivery.as_deref().unwrap_or(""),
						ad.ad_date.as_deref().unwrap_or(""),
						ad.avito_ad_id.as_str(),
						ad.title.as_deref().unwrap_or(""),
						ad.price.as_deref().unwrap_or(""),
						ad.link.as_deref().unwrap_or(""),
						ad.categories.as_deref().unwrap_or(""),
						ad.seller_id.as_deref().unwrap_or(""),
						ad.seller_name.as_deref().unwrap_or(""),
						ad.seller_type.as_deref().unwrap_or(""),
						ad.register_date.as_deref().unwrap_or(""),
						ad.answer_time.as_deref().unwrap_or(""),
						ad.rating.as_deref().unwrap_or(""),
						ad.reviews_count.as_deref().unwrap_or(""),
						ad.ads_count.as_deref().unwrap_or(""),
						ad.closed_ads_count.as_deref().unwrap_or(""),
						ad.photo_count.as_deref().unwrap_or(""),
						ad.address.as_deref().unwrap_or(""),
						ad.description.as_deref().unwrap_or(""),
					])
					.map_err(|e| {
						actix_web::error::ErrorInternalServerError(format!(
							"CSV write error: {:?}",
							e
						))
					})?;
			}

			// Get the CSV bytes
			let csv_bytes = writer
				.into_inner()
				.map_err(|e| {
					actix_web::error::ErrorInternalServerError(format!("CSV finish error: {:?}", e))
				})?
				.into_inner();

			// Generate filename using search_query and date
			let filename = if !ads.is_empty() {
				let first_ad = &ads[0];
				let search_query = first_ad
					.search_query
					.clone()
					.unwrap_or_else(|| "ads".to_string());
				// Replace invalid filename characters with underscores
				let sanitized_query = search_query
					.chars()
					.map(|c| match c {
						'/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0' => '_',
						_ => c,
					})
					.collect::<String>();
				let transliterated_query = Translit::convert(Some(sanitized_query));
				let date = chrono::Utc::now().format("%Y-%m-%d").to_string();
				format!("{}_{}.csv", transliterated_query, date)
			} else {
				let date = chrono::Utc::now().format("%Y-%m-%d").to_string();
				format!("ads_{}.csv", date)
			};

			// Create response with CSV content type
			Ok(HttpResponse::Ok()
				.content_type("text/csv")
				.append_header((
					"Content-Disposition",
					format!("attachment; filename=\"{}\"", filename),
				))
				.body(csv_bytes))
		}
		Err(e) => Ok(HttpResponse::InternalServerError()
			.json(serde_json::json!({"status": "error","message": format!("{:?}", e)}))),
	}
}
