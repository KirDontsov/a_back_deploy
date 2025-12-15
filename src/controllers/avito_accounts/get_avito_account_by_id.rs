use crate::{
	models::{AvitoAccount, AvitoAccountData, AvitoAccountResponse},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

use crate::jwt_auth::JwtMiddleware;
use crate::utils::encryption;

#[actix_web::get("/avito/accounts/{id}")]
pub async fn get_avito_account_by_id(
	path: web::Path<Uuid>,
	user: JwtMiddleware,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	let mut conn = data.db.get().unwrap();
	let account_id = path.into_inner();

	match crate::schema::avito_accounts::table
		.find(account_id)
		.first::<AvitoAccount>(&mut conn)
	{
		Ok(avito_account) => {
			// Check if the account belongs to the authenticated user
			if avito_account.user_id != user.user_id {
				return Ok(HttpResponse::Forbidden().json(json!({
					"status": "fail",
					"message": "You don't have permission to access this account"
				})));
			}

			// Attempt to decrypt credentials
			let mut avito_account = avito_account;
			match encryption::decrypt_avito_credentials(
				&avito_account.avito_client_secret,
				&avito_account.avito_client_id,
			) {
				Ok((decrypted_secret, decrypted_client_id)) => {
					avito_account.avito_client_secret = decrypted_secret;
					avito_account.avito_client_id = decrypted_client_id;
				}
				Err(e) => {
					eprintln!(
						"Failed to decrypt credentials for account {}: {}",
						avito_account.account_id, e
					);
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
		Err(diesel::result::Error::NotFound) => Ok(HttpResponse::NotFound().json(json!({
			"status": "fail",
			"message": "Avito account not found"
		}))),
		Err(e) => {
			eprintln!("Database error when fetching Avito account: {}", e);
			Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to fetch Avito account"
			})))
		}
	}
}
