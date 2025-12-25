use crate::jwt_auth::JwtMiddleware;
use crate::{
	models::{AvitoAccount, PaginationParams, PaginationResponse, ResponseWithPagination},
	AppState,
};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;

#[actix_web::get("/avito/accounts")]
pub async fn get_all_avito_accounts(
	user: JwtMiddleware,
	pagination: web::Query<PaginationParams>,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	let mut conn = data.db.get().unwrap();

	// Get total count for the specific user
	let total_count: i64 = crate::schema::avito_accounts::table
		.filter(crate::schema::avito_accounts::user_id.eq(user.user_id))
		.count()
		.get_result(&mut conn)
		.unwrap_or(0);

	// Get pagination parameters
	let page = pagination.page.unwrap_or(1).max(1);
	let limit = pagination.limit.unwrap_or(10).min(100); // max 100 per page
	let offset = (page - 1) * limit;

	// Calculate pages
	let pages = if limit > 0 {
		((total_count as f64) / (limit as f64)).ceil() as u32
	} else {
		1
	};

	// Query avito accounts for the authenticated user with pagination
	let avito_accounts: Vec<AvitoAccount> = crate::schema::avito_accounts::table
		.filter(crate::schema::avito_accounts::user_id.eq(user.user_id)) // Proper UUID comparison
		.limit(limit as i64)
		.offset(offset as i64)
		.load(&mut conn)
		.map_err(|e| {
			eprintln!("Database error when fetching Avito accounts: {}", e);
			actix_web::error::ErrorInternalServerError("Failed to fetch Avito accounts")
		})?;

	// Decrypt sensitive data before returning
	let mut decrypted_accounts: Vec<AvitoAccount> = Vec::new();
	for mut acc in avito_accounts {
		// Attempt to decrypt credentials
		match crate::utils::encryption::decrypt_avito_credentials(
			&acc.avito_client_secret,
			&acc.avito_client_id,
		) {
			Ok((decrypted_secret, decrypted_client_id)) => {
				acc.avito_client_secret = decrypted_secret;
				acc.avito_client_id = decrypted_client_id;
			}
			Err(e) => {
				eprintln!(
					"Failed to decrypt credentials for account {}: {}",
					acc.account_id, e
				);
				return Ok(HttpResponse::InternalServerError().json(json!({
					"status": "error",
					"message": "Failed to decrypt account credentials"
				})));
			}
		}
		decrypted_accounts.push(acc);
	}

	Ok(HttpResponse::Ok().json(ResponseWithPagination {
		status: "success".to_string(),
		data: decrypted_accounts,
		pagination: PaginationResponse {
			page,
			limit,
			total: total_count,
			pages,
		},
	}))
}
