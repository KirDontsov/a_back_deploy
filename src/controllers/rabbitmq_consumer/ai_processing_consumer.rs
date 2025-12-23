use futures::StreamExt;
use lapin::{
    options::{BasicConsumeOptions, QueueBindOptions, QueueDeclareOptions},
    types::FieldTable,
    Channel, Connection, ConnectionProperties,
};
use log;
use serde_json::Value;
use std::env;
use tokio::time::{sleep, Duration};

use crate::controllers::websocket::WebSocketConnections;

pub async fn start_ai_processing_consumer(
    ws_server: WebSocketConnections,
) {
    let rabbitmq_url =
        env::var("RABBITMQ_URL").unwrap_or_else(|_| "amqp://localhost:5672".to_string());

    loop {
        match Connection::connect(&rabbitmq_url, ConnectionProperties::default()).await {
            Ok(connection) => {
                match connection.create_channel().await {
                    Ok(channel) => {
                        println!("✅ Connected to RabbitMQ, starting AI processing consumer...");
                        
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

                        // Declare queue for AI processing responses
                        let queue = match channel
                            .queue_declare(
                                "ai_processing_responses",
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

                        // Bind queue to exchange with pattern to catch all result messages
                        if let Err(e) = channel
                            .queue_bind(
                                queue.name().as_str(),
                                "avito_exchange",
                                "result.*", // Wildcard to catch all result messages
                                QueueBindOptions::default(),
                                FieldTable::default(),
                            )
                            .await
                        {
                            eprintln!("Failed to bind queue with result pattern: {:?}", e);
                            log::error!(
                                "Failed to bind queue {} to exchange {} with routing key {}: {:?}",
                                queue.name(),
                                "avito_exchange",
                                "result.*",
                                e
                            );
                            sleep(Duration::from_secs(5)).await;
                            continue;
                        }

                        log::info!("Successfully bound queue {} to exchange {} with routing key result.*", queue.name(), "avito_exchange");

                        // Also bind to catch responses with specific pattern
                        if let Err(e) = channel
                            .queue_bind(
                                queue.name().as_str(),
                                "avito_exchange",
                                "#", // Catch-all binding for any messages
                                QueueBindOptions::default(),
                                FieldTable::default(),
                            )
                            .await
                        {
                            log::warn!("Failed to bind queue with catch-all pattern: {:?}", e);
                        } else {
                            log::info!("Successfully bound queue {} to exchange {} with catch-all routing pattern", queue.name(), "avito_exchange");
                        }

                        // Start consuming
                        let mut consumer = match channel
                            .basic_consume(
                                queue.name().as_str(),
                                "ai_processing_result_consumer",
                                BasicConsumeOptions::default(),
                                FieldTable::default(),
                            )
                            .await
                        {
                            Ok(consumer) => {
                                println!("✅ Started consuming from queue: {}, for AI processing results", queue.name().as_str());
                                log::info!("Started consuming from queue: {}, for AI processing results", queue.name().as_str());
                                consumer
                            }
                            Err(e) => {
                                eprintln!("Failed to start AI processing consumer: {:?}", e);
                                log::error!("Failed to start AI processing consumer: {:?}", e);
                                sleep(Duration::from_secs(5)).await;
                                continue;
                            }
                        };

                        println!("👂 Waiting for AI processing result messages...");
                        log::info!("Waiting for AI processing result messages on queue");

                        while let Some(delivery_result) = consumer.next().await {
                            match delivery_result {
                                Ok(delivery) => {
                                    let routing_key = delivery.routing_key.as_str();
                                    log::info!(
                                        "Received AI processing result delivery from RabbitMQ with routing key: {}",
                                        routing_key
                                    );

                                    match std::str::from_utf8(&delivery.data) {
                                        Ok(json_str) => {
                                            log::info!("Received AI processing result JSON string: {}", json_str);

                                            // Try to parse as a generic JSON Value first to extract user info
                                            match serde_json::from_str::<Value>(json_str) {
                                                Ok(json_value) => {
                                                    // Extract user_id from the message if present
                                                    let user_id = extract_user_id(&json_value);
                                                    let task_id = extract_task_id(&json_value);

                                                    println!(
                                                        "Received AI processing result: user_id={:?}, task_id={:?}",
                                                        user_id, task_id
                                                    );
                                                    log::info!(
                                                        "Received AI processing result: user_id={:?}, task_id={:?}",
                                                        user_id, task_id
                                                    );

                                                    // Broadcast the result to WebSocket clients
                                                    match serde_json::to_string(&json_value) {
                                                        Ok(json_str) => {
                                                            if let Some(user_uuid) = user_id {
                                                                log::info!("Broadcasting AI processing result to WebSocket for user: {}", user_uuid);
                                                                ws_server
                                                                    .broadcast_message_to_user(
                                                                        &user_uuid.to_string(),
                                                                        &json_str,
                                                                    )
                                                                    .await;
                                                            } else {
                                                                log::warn!("No user_id found in AI processing result, broadcasting to all");
                                                                ws_server
                                                                    .broadcast_message(&json_str)
                                                                    .await;
                                                            }
                                                        }
                                                        Err(e) => {
                                                            eprintln!("Failed to serialize AI processing result: {:?}", e);
                                                            log::error!("Failed to serialize AI processing result: {:?}", e);
                                                        }
                                                    }

                                                    // Acknowledge the message
                                                    if let Err(e) =
                                                        delivery.ack(Default::default()).await
                                                    {
                                                        eprintln!(
                                                            "Failed to acknowledge AI processing result message: {:?}",
                                                            e
                                                        );
                                                        log::error!(
                                                            "Failed to acknowledge AI processing result message: {:?}",
                                                            e
                                                        );
                                                    }
                                                }
                                                Err(parse_error) => {
                                                    log::warn!("Could not parse AI processing result message as JSON: {:?}", parse_error);
                                                    log::debug!(
                                                        "Message content that failed to parse: {}",
                                                        json_str
                                                    );

                                                    // Acknowledge the message to remove it from the queue
                                                    if let Err(e) =
                                                        delivery.ack(Default::default()).await
                                                    {
                                                        log::error!("Failed to acknowledge unparsable message: {:?}", e);
                                                    }
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "Failed to convert AI processing result message to string: {:?}",
                                                e
                                            );
                                            log::error!(
                                                "Failed to convert AI processing result message to string: {:?}",
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
                                    eprintln!("Error receiving AI processing result message: {:?}", e);
                                    log::error!("Error receiving AI processing result message: {:?}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to create RabbitMQ channel for AI processing consumer: {:?}", e);
                        // Sleep before trying to connect again
                        sleep(Duration::from_secs(5)).await;
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "Failed to connect to RabbitMQ for AI processing consumer: {:?}, retrying in 5 seconds...",
                    e
                );
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

// Helper function to extract user_id from JSON Value
fn extract_user_id(json_value: &Value) -> Option<uuid::Uuid> {
    // Look for user_id field in the JSON
    if let Some(user_id_str) = json_value.get("user_id").and_then(|v| v.as_str()) {
        uuid::Uuid::parse_str(user_id_str).ok()
    } else if let Some(user_id_val) = json_value.get("userId").or_else(|| json_value.get("UserID")) {
        // Also check for alternative field names
        user_id_val.as_str().and_then(|s| uuid::Uuid::parse_str(s).ok())
    } else {
        None
    }
}

// Helper function to extract task_id from JSON Value
fn extract_task_id(json_value: &Value) -> Option<uuid::Uuid> {
    // Look for task_id field in the JSON
    if let Some(task_id_str) = json_value.get("task_id").and_then(|v| v.as_str()) {
        uuid::Uuid::parse_str(task_id_str).ok()
    } else if let Some(task_id_val) = json_value.get("taskId").or_else(|| json_value.get("TASK_ID")) {
        // Also check for alternative field names
        task_id_val.as_str().and_then(|s| uuid::Uuid::parse_str(s).ok())
    } else {
        None
    }
}