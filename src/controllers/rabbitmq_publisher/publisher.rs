use lapin::{
	options::{BasicPublishOptions, ExchangeDeclareOptions},
	types::FieldTable,
	Channel, Connection, ConnectionProperties, ExchangeKind,
};
use serde_json::Value;
use std::env;

pub async fn establish_rabbitmq_connection() -> Result<Channel, Box<dyn std::error::Error>> {
	use log;

	let rabbitmq_url =
		env::var("RABBITMQ_URL").unwrap_or_else(|_| "amqp://localhost:5672".to_string());

	log::info!("Connecting to RabbitMQ at: {}", rabbitmq_url);

	let connection = Connection::connect(&rabbitmq_url, ConnectionProperties::default())
		.await
		.map_err(|e| {
			log::error!("Failed to connect to RabbitMQ at {}: {:?}", rabbitmq_url, e);
			Box::new(e) as Box<dyn std::error::Error>
		})?;
	log::info!("Connected to RabbitMQ, creating channel...");

	let channel = connection.create_channel().await.map_err(|e| {
		log::error!("Failed to create RabbitMQ channel: {:?}", e);
		Box::new(e) as Box<dyn std::error::Error>
	})?;
	log::info!("Created RabbitMQ channel");

	// Declare exchange
	log::info!("Declaring exchange: avito_exchange");
	let result = channel
		.exchange_declare(
			"avito_exchange",
			ExchangeKind::Topic,
			ExchangeDeclareOptions {
				durable: true,
				auto_delete: false,
				..ExchangeDeclareOptions::default()
			},
			FieldTable::default(),
		)
		.await;

	match result {
		Ok(_) => log::info!("Declared exchange: avito_exchange"),
		Err(e) => {
			log::error!("Failed to declare exchange: avito_exchange, error: {:?}", e);
			return Err(Box::new(e));
		}
	}

	Ok(channel)
}

pub async fn publish_avito_request(channel: &Channel, message: &Value) -> Result<(), lapin::Error> {
	use log;

	let payload = serde_json::to_string(message).unwrap_or_default();
	log::info!("Publishing message to RabbitMQ: exchange='avito_exchange', routing_key='task.crawl.avito_request', payload={}", payload);

	let result = channel
		.basic_publish(
			"avito_exchange",
			"task.crawl.avito_request",
			BasicPublishOptions::default(),
			payload.as_bytes(),
			lapin::BasicProperties::default(),
		)
		.await;

	match &result {
		Ok(_) => log::info!("Successfully sent message to RabbitMQ"),
		Err(e) => log::error!("Failed to send message to RabbitMQ: {:?}", e),
	}

	result?;

	Ok(())
}
