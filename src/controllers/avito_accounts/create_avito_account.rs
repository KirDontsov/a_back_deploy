use crate::models::AvitoAccount;
use crate::{
	models::{AvitoAccountData, AvitoAccountResponse, CreateAvitoAccount},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;

#[actix_web::post("/avito/accounts")]
pub async fn create_avito_account(
	body: web::Json<CreateAvitoAccount>,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	let mut conn = data.db.get().unwrap();

	// Validate required fields
	if body.avito_client_secret.is_empty() || body.avito_client_id.is_empty() {
		return Ok(HttpResponse::BadRequest().json(json!({
			"status": "error",
			"message": "Avito client secret and client ID are required"
		})));
	}

	// Encrypt sensitive data before storing
	let encrypted_secret = match encrypt_field(&body.avito_client_secret) {
		Ok(encrypted) => encrypted,
		Err(e) => {
			eprintln!("Failed to encrypt avito_client_secret: {}", e);
			return Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to encrypt sensitive data"
			})));
		}
	};

	let encrypted_client_id = match encrypt_field(&body.avito_client_id) {
		Ok(encrypted) => encrypted,
		Err(e) => {
			eprintln!("Failed to encrypt avito_client_id: {}", e);
			return Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to encrypt sensitive data"
			})));
		}
	};

	let new_avito_account = CreateAvitoAccount {
		user_id: body.user_id, // Keep as Uuid type
		client_id: body.client_id.clone(),
		avito_client_secret: encrypted_secret,
		avito_client_id: encrypted_client_id,
		is_connected: body.is_connected,
	};

	match diesel::insert_into(crate::schema::avito_accounts::table)
		.values(&new_avito_account)
		.get_result::<AvitoAccount>(&mut conn)
	{
		Ok(mut avito_account) => {
			// Decrypt credentials for the response
			match crate::utils::encryption::decrypt_avito_credentials(
				&avito_account.avito_client_secret,
				&avito_account.avito_client_id,
			) {
				Ok((decrypted_secret, decrypted_client_id)) => {
					avito_account.avito_client_secret = decrypted_secret;
					avito_account.avito_client_id = decrypted_client_id;
				}
				Err(e) => {
					eprintln!("Failed to decrypt credentials after creation: {}", e);
					return Ok(HttpResponse::InternalServerError().json(json!({
						"status": "error",
						"message": "Failed to decrypt account credentials"
					})));
				}
			}

			Ok(HttpResponse::Ok().json(AvitoAccountResponse {
				status: "success".to_string(),
				data: AvitoAccountData { avito_account },
			}))
		}
		Err(e) => {
			eprintln!("Database error when creating Avito account: {}", e);
			Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to create Avito account"
			})))
		}
	}
}

// Helper function to encrypt a field
fn encrypt_field(field: &str) -> Result<String, Box<dyn std::error::Error>> {
	// Global key for encryption (in production, this should be stored securely)
	static ENCRYPTION_KEY: [u8; 32] = [
		1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
		26, 27, 28, 29, 30, 31, 32,
	];

	let iv = crate::utils::encryption::generate_iv();
	let encrypted_data = crate::utils::encryption::encrypt_data(field, &ENCRYPTION_KEY, &iv);
	Ok(crate::utils::encryption::combine_iv_and_data(
		&iv,
		&encrypted_data,
	))
}
