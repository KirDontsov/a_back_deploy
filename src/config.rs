use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config {
	pub database_url: String,
	pub jwt_secret: String,
	pub server_port: u16,
}

impl Config {
	pub fn init() -> Self {
		dotenv().ok();

		Config {
			database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
			jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
			server_port: env::var("SERVER_PORT")
				.unwrap_or_else(|_| "8081".to_string())
				.parse()
				.expect("SERVER_PORT must be a valid number"),
		}
	}
}
