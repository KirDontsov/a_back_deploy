use futures::StreamExt;
use lapin::{
	options::{BasicConsumeOptions, QueueBindOptions, QueueDeclareOptions},
	types::FieldTable,
	Channel, Connection, ConnectionProperties, Queue,
};
use log;
use serde_json::Value;
use std::env;
use tokio::time::{sleep, Duration};

use super::message::ProgressUpdateMessage;
use crate::models::{AvitoRequest, CreateAvitoRequest};
use crate::schema::avito_requests;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;

// Import WebSocket server to broadcast messages
use crate::controllers::websocket::WebSocketConnections;

pub async fn start_rabbitmq_consumer(
	db_pool: diesel::r2d2::Pool<ConnectionManager<PgConnection>>,
	ws_server: WebSocketConnections,
) -> ! {
	let rabbitmq_url =
		env::var("RABBITMQ_URL").unwrap_or_else(|_| "amqp://localhost:5672".to_string());

	loop {
		match Connection::connect(&rabbitmq_url, ConnectionProperties::default()).await {
			Ok(connection) => {
				match connection.create_channel().await {
					Ok(channel) => {
						println!("✅ Connected to RabbitMQ, starting consumer...");

						// Declare exchange
						if let Err(e) = channel
							.exchange_declare(
								"avito_exchange",
								lapin::ExchangeKind::Topic,
								lapin::options::ExchangeDeclareOptions {
									durable: true,
									auto_delete: false,
									..lapin::options::ExchangeDeclareOptions::default()
								},
								FieldTable::default(),
							)
							.await
						{
							eprintln!("Failed to declare exchange: {:?}", e);
							sleep(Duration::from_secs(5)).await;
							continue;
						}

						// Declare queue
						let queue = match channel
							.queue_declare(
								"avito_progress_updates",
								QueueDeclareOptions {
									durable: true,
									..QueueDeclareOptions::default()
								},
								FieldTable::default(),
							)
							.await
						{
							Ok(queue) => queue,
							Err(e) => {
								eprintln!("Failed to declare queue: {:?}", e);
								sleep(Duration::from_secs(5)).await;
								continue;
							}
						};

						// Bind queue to exchange with specific routing key
						if let Err(e) = channel
							.queue_bind(
								queue.name().as_str(),
								"avito_exchange",
								"task.progress.update",
								QueueBindOptions::default(),
								FieldTable::default(),
							)
							.await
						{
							eprintln!("Failed to bind queue: {:?}", e);
							log::error!(
								"Failed to bind queue {} to exchange {} with routing key {}: {:?}",
								queue.name(),
								"avito_exchange",
								"task.progress.update",
								e
							);
							sleep(Duration::from_secs(5)).await;
							continue;
						}
						log::info!("Successfully bound queue {} to exchange {} with routing key task.progress.update", queue.name(), "avito_exchange");

						// Also bind with a wildcard pattern to catch any similar routing keys
						if let Err(e) = channel
							.queue_bind(
								queue.name().as_str(),
								"avito_exchange",
								"task.progress.*",
								QueueBindOptions::default(),
								FieldTable::default(),
							)
							.await
						{
							eprintln!("Failed to bind queue with wildcard: {:?}", e);
							log::warn!(
								"Failed to bind queue {} to exchange {} with routing key {}: {:?}",
								queue.name(),
								"avito_exchange",
								"task.progress.*",
								e
							);
							// Don't exit on wildcard binding failure, continue with the primary binding
						} else {
							log::info!("Successfully bound queue {} to exchange {} with routing key task.progress.*", queue.name(), "avito_exchange");
						}

						// As a fallback, bind to catch all messages in case routing key is completely different
						if let Err(e) = channel
							.queue_bind(
								queue.name().as_str(),
								"avito_exchange",
								"#", // Catch-all binding
								QueueBindOptions::default(),
								FieldTable::default(),
							)
							.await
						{
							log::warn!("Failed to bind queue with catch-all pattern: {:?}", e);
							// Don't exit on catch-all binding failure, continue with other bindings
						} else {
							log::info!("Successfully bound queue {} to exchange {} with catch-all routing pattern", queue.name(), "avito_exchange");
						}

						// Start consuming
						let mut consumer = match channel
							.basic_consume(
								queue.name().as_str(),
								"avito_progress_consumer",
								BasicConsumeOptions::default(),
								FieldTable::default(),
							)
							.await
						{
							Ok(consumer) => {
								println!("✅ Started consuming from queue: {}, with routing key: task.progress.update", queue.name().as_str());
								log::info!("Started consuming from queue: {}, with routing key: task.progress.update", queue.name().as_str());
								consumer
							}
							Err(e) => {
								eprintln!("Failed to start consumer: {:?}", e);
								log::error!("Failed to start consumer: {:?}", e);
								sleep(Duration::from_secs(5)).await;
								continue;
							}
						};

						println!("👂 Waiting for progress update messages...");
						log::info!("Waiting for progress update messages on queue");

						while let Some(delivery_result) = consumer.next().await {
							match delivery_result {
								Ok(delivery) => {
									let routing_key = delivery.routing_key.as_str();
									log::info!(
										"Received delivery from RabbitMQ with routing key: {}",
										routing_key
									);
									match std::str::from_utf8(&delivery.data) {
										Ok(json_str) => {
											log::info!("Received JSON string: {}", json_str);
											// Try to parse as ProgressUpdateMessage
											match serde_json::from_str::<ProgressUpdateMessage>(
												json_str,
											) {
												Ok(progress_msg) => {
													println!(
														"Received progress update: {:?}",
														progress_msg
													);
													log::info!(
														"Received progress update: {:?}",
														progress_msg
													);

													// Update database with progress information
													update_database_progress(
														&db_pool,
														&progress_msg,
													)
													.await;

													// Broadcast the progress update to WebSocket clients
													match serde_json::to_string(&progress_msg) {
														Ok(json_str) => {
															log::info!("Broadcasting progress update to WebSocket for request: {}", progress_msg.request_id);
															ws_server
																.broadcast_message_to_request(
																	&progress_msg
																		.request_id
																		.to_string(),
																	&json_str,
																)
																.await;
														}
														Err(e) => {
															eprintln!("Failed to serialize progress message: {:?}", e);
															log::error!("Failed to serialize progress message: {:?}", e);
														}
													}

													// Acknowledge the message
													if let Err(e) =
														delivery.ack(Default::default()).await
													{
														eprintln!(
															"Failed to acknowledge message: {:?}",
															e
														);
														log::error!(
															"Failed to acknowledge message: {:?}",
															e
														);
													}
												}
												Err(parse_error) => {
													// This might not be a progress update message, log and acknowledge anyway
													log::warn!("Received message that is not a progress update (routing key: {}): {:?}", routing_key, parse_error);
													log::debug!(
														"Message content that failed to parse: {}",
														json_str
													);

													// Acknowledge the message to remove it from the queue since it's not our target message type
													if let Err(e) =
														delivery.ack(Default::default()).await
													{
														log::error!("Failed to acknowledge non-progress message: {:?}", e);
													}
												}
											}
										}
										Err(e) => {
											eprintln!(
												"Failed to convert message to string: {:?}",
												e
											);
											log::error!(
												"Failed to convert message to string: {:?}",
												e
											);

											// Acknowledge the message to remove it from the queue
											if let Err(e) = delivery.ack(Default::default()).await {
												log::error!("Failed to acknowledge invalid UTF-8 message: {:?}", e);
											}
										}
									}
								}
								Err(e) => {
									eprintln!("Error receiving message: {:?}", e);
									log::error!("Error receiving message: {:?}", e);
								}
							}
						}
					}
					Err(e) => {
						eprintln!("Failed to create RabbitMQ channel: {:?}", e);
						// Sleep before trying to connect again
						sleep(Duration::from_secs(5)).await;
					}
				}
			}
			Err(e) => {
				eprintln!(
					"Failed to connect to RabbitMQ: {:?}, retrying in 5 seconds...",
					e
				);
				sleep(Duration::from_secs(5)).await;
			}
		}
	}
}

// Since the schema doesn't have progress fields, I'll need to extend the model to track progress
// For now, I'll implement a solution that stores progress information in a temporary way
// In a real-world scenario, you'd want to add progress fields to the avito_requests table
// or create a separate progress tracking table

use diesel::sql_types::{Double, Integer, Text, Timestamp, Uuid as DieselUuid};
use diesel::{sql_query, RunQueryDsl};

#[derive(QueryableByName, Debug)]
struct ProgressInfo {
	#[diesel(sql_type = DieselUuid)]
	request_id: uuid::Uuid,
	#[diesel(sql_type = Double)]
	progress: f64,
	#[diesel(sql_type = Text)]
	status: String,
	#[diesel(sql_type = Text)]
	message: String,
	#[diesel(sql_type = Integer)]
	total_ads: i32,
	#[diesel(sql_type = Integer)]
	current_ads: i32,
	#[diesel(sql_type = Timestamp)]
	updated_at: chrono::NaiveDateTime,
}

async fn update_database_progress(
	db_pool: &diesel::r2d2::Pool<ConnectionManager<PgConnection>>,
	progress_msg: &ProgressUpdateMessage,
) {
	use crate::schema::avito_requests::dsl::*;

	let mut conn = match db_pool.get() {
		Ok(conn) => conn,
		Err(e) => {
			eprintln!("Failed to get database connection: {:?}", e);
			return;
		}
	};

	// Since we don't have progress fields in the schema, we'll just verify that the request exists
	// In a real implementation, you'd want to add progress fields to the database schema
	// or create a separate progress tracking table

	// Check if the avito request exists
	let existing_request = match avito_requests
		.find(&progress_msg.request_id)
		.first::<AvitoRequest>(&mut conn)
		.optional()
	{
		Ok(result) => result,
		Err(e) => {
			eprintln!("Failed to query database: {:?}", e);
			return;
		}
	};

	if let Some(request_record) = existing_request {
		println!(
			"Found existing request for ID: {}",
			request_record.request_id
		);

		// In a production implementation, you would update progress fields in the database
		// For now, we'll just log the progress update

		// Example SQL to add progress tracking (this would need to be added to the schema):
		// ALTER TABLE avito_requests ADD COLUMN progress DOUBLE PRECISION DEFAULT 0.0;
		// ALTER TABLE avito_requests ADD COLUMN progress_status VARCHAR(50) DEFAULT 'pending';
		// ALTER TABLE avito_requests ADD COLUMN progress_message TEXT;
		// ALTER TABLE avito_requests ADD COLUMN total_ads INTEGER DEFAULT 0;
		// ALTER TABLE avito_requests ADD COLUMN current_ads INTEGER DEFAULT 0;

		// Placeholder for future implementation
		println!(
			"Progress update for request {}: {}% complete, {} ads processed out of {}",
			progress_msg.request_id,
			progress_msg.progress,
			progress_msg.current_ads,
			progress_msg.total_ads
		);
	} else {
		eprintln!(
			"Request ID {} not found in database",
			progress_msg.request_id
		);
	}
}
