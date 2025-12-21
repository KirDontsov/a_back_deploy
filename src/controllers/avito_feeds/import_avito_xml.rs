use crate::{
	jwt_auth::JwtMiddleware,
	models::{AvitoFeed, CreateAvitoFeed, XmlAd},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ImportAvitoXmlRequest {
	pub account_id: Uuid,
	pub xml_url: String,
}

#[actix_web::post("/avito/feeds/import_xml")]
pub async fn import_avito_xml(
	body: web::Json<ImportAvitoXmlRequest>,
	data: web::Data<AppState>,
	_: JwtMiddleware,
) -> Result<HttpResponse> {
	let xml_url = &body.xml_url;
	let account_id = body.account_id;

	// Fetch XML data
	let client = match Client::builder().timeout(Duration::from_secs(30)).build() {
		Ok(client) => client,
		Err(e) => {
			log::error!("Failed to build HTTP client: {:?}", e);
			return Ok(
				actix_web::HttpResponse::InternalServerError().json(serde_json::json!({
					"status": "error",
					"message": format!("Failed to build HTTP client: {}", e)
				})),
			);
		}
	};

	let response = match client.get(xml_url).send().await {
		Ok(response) => response,
		Err(e) => {
			log::error!("Failed to fetch XML: {:?}", e);
			return Ok(
				actix_web::HttpResponse::InternalServerError().json(serde_json::json!({
					"status": "error",
					"message": format!("Failed to fetch XML: {}", e)
				})),
			);
		}
	};

	if !response.status().is_success() {
		log::error!("Failed to fetch XML: Status {}", response.status());
		return Ok(
			actix_web::HttpResponse::InternalServerError().json(serde_json::json!({
				"status": "error",
				"message": format!("Failed to fetch XML: Status {}", response.status())
			})),
		);
	}

	let xml_data = match response.text().await {
		Ok(xml_data) => xml_data,
		Err(e) => {
			log::error!("Failed to read response: {:?}", e);
			return Ok(
				actix_web::HttpResponse::InternalServerError().json(serde_json::json!({
					"status": "error",
					"message": format!("Failed to read response: {}", e)
				})),
			);
		}
	};

	// Parse XML and extract ads
	println!("Parsing XML data with length: {}", xml_data.len());
	let ads = match parse_xml_ads(&xml_data) {
		Ok(ads) => ads,
		Err(e) => {
			log::error!("Failed to parse XML: {:?}", e);
			return Ok(
				actix_web::HttpResponse::InternalServerError().json(serde_json::json!({
					"status": "error",
					"message": format!("Failed to parse XML: {}", e)
				})),
			);
		}
	};
	println!("Parsed {} ads from XML", ads.len());

	// Print debug information about the first few ads
	for (i, ad) in ads.iter().take(3).enumerate() {
		println!("Ad {}: id={}, fields={}", i, ad.id, ad.fields.len());
		if let Some(images) = ad.fields.get("Images") {
			println!(" Images field: {}", images);
		} else {
			println!("  No Images field found");
		}
	}

	println!("Parsed {} ads from XML", ads.len());

	let mut conn = data.db.get().unwrap();

	// Create feed entry
	let feed_id = Uuid::new_v4();
	let new_feed = CreateAvitoFeed {
		account_id,
		category: "IMPORT".to_string(),
	};

	let avito_feed_result: Result<AvitoFeed, diesel::result::Error> =
		diesel::insert_into(crate::schema::avito_feeds::table)
			.values(new_feed)
			.get_result(&mut conn);

	match avito_feed_result {
		Ok(avito_feed) => Ok(HttpResponse::Ok().json(serde_json::json!({
			"status": "success",
			"message": "Import completed successfully",
			"feed_id": avito_feed.feed_id,
			"ads_processed": ads.len()
		}))),
		Err(e) => {
			log::error!("Failed to create feed: {:?}", e);
			Ok(
				actix_web::HttpResponse::InternalServerError().json(serde_json::json!({
					"status": "error",
					"message": format!("Failed to create feed: {}", e)
				})),
			)
		}
	}
}

pub fn parse_xml_ads(xml_data: &str) -> Result<Vec<XmlAd>, String> {
	let mut reader = Reader::from_str(xml_data);
	let mut ads = Vec::new();
	let mut buf = Vec::new();
	let mut current_ad: Option<XmlAd> = None;
	let mut current_path = Vec::new();
	let mut current_values = String::new();
	let mut in_ad = false;
	let mut delivery_buffer = Vec::new();
	let mut images_buffer: Vec<String> = Vec::new();

	loop {
		match reader.read_event_into(&mut buf) {
			Ok(Event::Start(e)) => {
				let name = std::str::from_utf8(e.name().as_ref())
					.map_err(|e| format!("UTF-8 error: {}", e))?
					.to_string();

				// println!("Start element: {}, current path: {:?}", name, current_path);
				current_path.push(name.clone());

				if name == "Ad" {
					in_ad = true;
					current_ad = Some(XmlAd {
						id: String::new(),
						fields: HashMap::new(),
					});
					delivery_buffer.clear();
					images_buffer.clear();
				}

				current_values.clear();
			}
			Ok(Event::Text(e)) => {
				// Extract text content directly from the bytes
				let text = std::str::from_utf8(e.into_inner().as_ref())
					.map_err(|e| format!("UTF-8 error: {}", e))?
					.to_string();

				if in_ad && !&text.trim().is_empty() {
					current_values.push_str(&text);
				}
			}
			Ok(Event::CData(e)) => {
				// Handle CDATA content (for Description)
				let text = std::str::from_utf8(e.as_ref())
					.map_err(|e| format!("UTF-8 error: {}", e))?
					.to_string();

				if in_ad {
					current_values.push_str(&text);
				}
			}
			Ok(Event::Empty(e)) => {
				let name = std::str::from_utf8(e.name().as_ref())
					.map_err(|e| format!("UTF-8 error: {}", e))?
					.to_string();

				// Handle Image tags with attributes
				if name == "Image" && current_path.contains(&"Images".to_string()) {
					// Extract the url attribute directly and add to images_buffer
					for attr_result in e.attributes() {
						if let Ok(attr) = attr_result {
							if attr.key.as_ref() == b"url" {
								if let Ok(url) = std::str::from_utf8(&attr.value) {
									images_buffer.push(url.to_string());
								}
							}
						}
					}
				}
				// Handle other empty elements (fallback)
				else {
					let text = std::str::from_utf8(e.as_ref())
						.map_err(|e| format!("UTF-8 error: {}", e))?
						.to_string();

					if in_ad {
						current_values.push_str(&text);
					}
				}
			}
			Ok(Event::End(e)) => {
				let name = std::str::from_utf8(e.name().as_ref())
					.map_err(|e| format!("UTF-8 error: {}", e))?
					.to_string();

				if let Some(ad) = &mut current_ad {
					// Special handling for Delivery - store as comma-separated options
					if name == "Delivery" && !delivery_buffer.is_empty() {
						ad.fields
							.insert("Delivery".to_string(), delivery_buffer.join(","));
						delivery_buffer.clear();
					}
					// Special handling for Option elements inside Delivery
					else if name == "Option" && current_path.contains(&"Delivery".to_string()) {
						if !current_values.trim().is_empty() {
							delivery_buffer.push(current_values.trim().to_string());
						}
					} else if name == "Images" {
						// Store image URLs when closing Images tag
						if !images_buffer.is_empty() {
							ad.fields
								.insert("Images".to_string(), images_buffer.join(","));
							images_buffer.clear();
						}
					} else if name == "Image" && current_path.contains(&"Images".to_string()) {
						// For non-empty Image tags, add their text content to images_buffer
						if !current_values.trim().is_empty() {
							images_buffer.push(current_values.trim().to_string());
						}
					}
					// Store other field values if not empty
					else if !current_values.trim().is_empty() && current_path.len() > 1 {
						let field_name = current_path.last().unwrap().clone();
						// Skip storing individual Image and Option elements as they're handled specially
						if field_name != "Image" && field_name != "Option" {
							ad.fields
								.insert(field_name, current_values.trim().to_string());
						}
					}

					// Special handling for Id field
					if name == "Id" {
						if let Some(id_value) = ad.fields.get("Id") {
							ad.id = id_value.clone();
						}
					}
				}

				// If this is the end of an Ad element, add it to the list
				if name == "Ad" {
					in_ad = false;
					if let Some(ad) = current_ad.take() {
						if !ad.id.is_empty() {
							ads.push(ad);
						}
					}
				}

				// println!("End element: {}, current path: {:?}", name, current_path);
				current_path.pop();
				current_values.clear();
			}
			Ok(Event::Eof) => break,
			Err(e) => return Err(format!("XML parse error: {}", e)),
			_ => (),
		}

		// Clear the buffer to prevent re-processing the same event
		buf.clear();
	}

	println!("Finished parsing {} ads", ads.len());
	Ok(ads)
}
