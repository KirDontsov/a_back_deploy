use crate::controllers::rabbitmq_publisher::publisher::publish_avito_request;
use crate::jwt_auth::JwtMiddleware;
use crate::{
	models::{
		AvitoRequest, AvitoRequestData, AvitoRequestResponse, CreateAvitoRequestJson,
		CreateAvitoRequestWithUserId,
	},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use log;
use serde_json::json;
use uuid::Uuid;

// Helper function to filter avito request record (removing sensitive data if needed)
fn filter_add_avito_request_record(avito_request: &AvitoRequest) -> serde_json::Value {
	json!({
		"request_id": avito_request.request_id,
		"request": avito_request.request,
		"city": avito_request.city,
		"coords": avito_request.coords,
		"radius": avito_request.radius,
		"district": avito_request.district,
		"created_ts": avito_request.created_ts,
		"updated_ts": avito_request.updated_ts,
		"user_id": avito_request.user_id
	})
}

// Create avito request
#[actix_web::post("/avito_requests")]
pub async fn create_avito_request(
	data: web::Data<AppState>,
	new_request: web::Json<CreateAvitoRequestJson>,
	user: JwtMiddleware,
) -> Result<HttpResponse> {
	eprintln!("DEBUG: create_avito_request handler called!");
	log::info!(
		"create_avito_request handler called with request: {:?}",
		new_request
	);
	log::info!(
		"RabbitMQ channel state at function start: {}",
		if data.rabbitmq_channel.is_some() {
			"SOME"
		} else {
			"NONE"
		}
	);
	let mut conn = data.db.get().unwrap();

	// Create a struct that includes the user_id from JWT middleware
	let new_request_with_user_id = crate::models::CreateAvitoRequestWithUserId {
		request: new_request.request.clone(),
		city: new_request.city.clone(),
		coords: new_request.coords.clone(),
		radius: new_request.radius.clone(),
		district: new_request.district.clone(),
		user_id: user.user_id,
	};

	let avito_request = diesel::insert_into(crate::schema::avito_requests::table)
		.values(new_request_with_user_id)
		.get_result::<AvitoRequest>(&mut conn);

	match avito_request {
		Ok(avito_request) => {
			// Prepare message for RabbitMQ with proper string conversions
			let message = json!({
				"request_id": avito_request.request_id,
				"user_id": user.user_id,
				"request": avito_request.request,
				"city": avito_request.city.as_deref().unwrap_or(""),
				"coords": avito_request.coords.as_deref().unwrap_or(""),
				"radius": avito_request.radius.as_deref().unwrap_or(""),
				"district": avito_request.district.as_deref().unwrap_or(""),
				"created_ts": avito_request.created_ts.to_string(),
			});

			// Publish to RabbitMQ if channel is available
			log::info!(
				"Checking RabbitMQ channel availability for request_id: {}",
				avito_request.request_id
			);
			log::info!(
				"RabbitMQ channel state: {}",
				if data.rabbitmq_channel.is_some() {
					"SOME"
				} else {
					"NONE"
				}
			);
			match &data.rabbitmq_channel {
				Some(channel) => {
					eprintln!("DEBUG: RabbitMQ channel available, attempting to publish message");
					log::info!("RabbitMQ channel is available, attempting to publish avito request with request_id: {}", avito_request.request_id);
					match publish_avito_request(channel, &message).await {
						Ok(_) => {
							eprintln!("DEBUG: Successfully published avito request to RabbitMQ");
							log::info!("Successfully published avito request to RabbitMQ with request_id: {}", avito_request.request_id);
							let avito_request_response = serde_json::json!({
								"status": "success",
								"data": serde_json::json!({
									"avito_request": filter_add_avito_request_record(&avito_request.clone())
								})
							});
							Ok(HttpResponse::Ok().json(avito_request_response))
						}
						Err(e) => {
							eprintln!("DEBUG: Failed to publish message to RabbitMQ: {}", e);
							log::error!("Failed to publish message to RabbitMQ: {}", e);
							// You might want to handle this differently - maybe still return success
							// but log the error, or return a partial success response
							Ok(HttpResponse::Accepted().json(serde_json::json!({
								"status": "success",
								"message": "Request created but notification failed"
							})))
						}
					}
				}
				None => {
					eprintln!("DEBUG: RabbitMQ channel not available, skipping message publishing");
					log::warn!("RabbitMQ channel is not available, skipping message publishing for request_id: {}", avito_request.request_id);
					// RabbitMQ channel is not available, return success without publishing
					let avito_request_response = serde_json::json!({
						"status": "success",
						"data": serde_json::json!({
							"avito_request": filter_add_avito_request_record(&avito_request.clone())
						})
					});
					Ok(HttpResponse::Ok().json(avito_request_response))
				}
			}
		}
		Err(e) => {
			log::error!("Failed to create avito request in database: {:?}", e);
			Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to create avito request"
			})))
		}
	}
}
