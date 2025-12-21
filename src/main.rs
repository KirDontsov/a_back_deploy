#![feature(trivial_bounds)]
mod config;
mod controllers;
mod jwt_auth;
mod models;
mod schema;
mod utils;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use config::Config;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use lapin::Channel;
use crate::controllers::rabbitmq_consumer::start_rabbitmq_consumer;
use crate::controllers::rabbitmq_publisher::publisher::establish_rabbitmq_connection;
use crate::controllers::websocket::{websocket_handler, WebSocketConnections};

pub struct AppState {
	db: r2d2::Pool<ConnectionManager<diesel::PgConnection>>,
	env: Config,
	pub rabbitmq_channel: Option<Channel>,
	ws_server: WebSocketConnections,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	if std::env::var_os("RUST_LOG").is_none() {
		std::env::set_var("RUST_LOG", "actix_web=info,lapin=info");
	}
	dotenv().ok();
	env_logger::init();

	let config = Config::init();

	let manager = ConnectionManager::<diesel::PgConnection>::new(&config.database_url);
	let pool = r2d2::Pool::builder()
		.max_size(10)
		.build(manager)
		.expect("Failed to create database pool");

	// Test the database connection
	match pool.get() {
		Ok(mut connection) => {
			match diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>("1"))
				.get_result::<i32>(&mut connection)
			{
				Ok(_) => println!("✅ Connection to the database is successful!"),
				Err(err) => {
					eprintln!("🔥 Failed to connect to the database: {:?}", err);
					std::process::exit(1);
				}
			};
		}
		Err(err) => {
			eprintln!("🔥 Failed to get database connection from pool: {:?}", err);
			std::process::exit(1);
		}
	};

	// Establish RabbitMQ connection (optional)
	let rabbitmq_channel = match establish_rabbitmq_connection().await {
		Ok(channel) => {
			println!("✅ Connected to RabbitMQ successfully!");
			Some(channel)
		}
		Err(e) => {
			eprintln!("⚠️  Warning: Failed to connect to RabbitMQ: {:?}. The application will continue to run but without RabbitMQ functionality.", e);
			None
		}
	};

	// Create WebSocket server instance
	let ws_server = WebSocketConnections::new();
	let ws_server_data = web::Data::new(ws_server.clone());

	// Start RabbitMQ consumer with WebSocket server
	let ws_server_clone = ws_server.clone();
	let pool_clone = pool.clone();
	tokio::spawn(async move { start_rabbitmq_consumer(pool_clone, ws_server_clone).await });

	println!("✅ Server started successfully on http://0.0.0.0:8081");

	HttpServer::new(move || {
		App::new()
			.app_data(web::Data::new(AppState {
				db: pool.clone(),
				env: config.clone(),
				rabbitmq_channel: rabbitmq_channel.clone(),
				ws_server: ws_server.clone(),
			}))
			.app_data(ws_server_data.clone())
			.service(web::resource("/api/ws").route(web::get().to(
				|req: HttpRequest, body: web::Payload, data: web::Data<WebSocketConnections>| async move {
					websocket_handler(req, body, data).await
				},
			)))
			.configure(controllers::config::config)
			.wrap(Cors::permissive())
			.wrap(Logger::default())
			.route(
				"/",
				web::get().to(|| async { HttpResponse::Ok().body("Actix-web server is running!") }),
			)
	})
	.bind(("0.0.0", 8081))?
	.run()
	.await
}
