#![feature(trivial_bounds)]
mod config;
mod controllers;
mod jwt_auth;
mod models;
mod schema;
mod utils;

use actix_cors::Cors;
use actix_web::dev::ServiceRequest;
use actix_web::http;
use actix_web::middleware::Logger;
use actix_web::Error as ActixWebError;
use actix_web::{web, App, HttpResponse, HttpServer};
// use actix_web_grants::GrantsMiddleware;
use config::Config;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};

pub struct AppState {
	db: r2d2::Pool<ConnectionManager<diesel::PgConnection>>,
	env: Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	if std::env::var_os("RUST_LOG").is_none() {
		std::env::set_var("RUST_LOG", "actix_web=info");
	}
	dotenv().ok();
	env_logger::init();

	let config = Config::init();

	let manager = ConnectionManager::<diesel::PgConnection>::new(&config.database_url);
	let pool = r2d2::Pool::builder()
		.max_size(10)
		.build(manager)
		.expect("Failed to create database pool");

	// Test the connection
	use diesel::Connection;
	match pool.get() {
	    Ok(mut connection) => {
	        match diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>("1")).get_result::<i32>(&mut connection) {
	            Ok(_) => println!("✅ Connection to the database is successful!"),
	            Err(err) => {
	                eprintln!("🔥 Failed to connect to the database: {:?}", err);
	                std::process::exit(1);
	            }
	        };
	    },
	    Err(err) => {
	        eprintln!("🔥 Failed to get database connection from pool: {:?}", err);
	        std::process::exit(1);
	    }
	};

	println!("✅ Server started successfully on http://0.0.0:8081");

	HttpServer::new(move || {
			App::new()
				.app_data(web::Data::new(AppState {
					db: pool.clone(),
					env: config.clone(),
				}))
				.configure(controllers::config)
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
