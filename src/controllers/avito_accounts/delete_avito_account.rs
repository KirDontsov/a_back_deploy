use crate::{models::AvitoAccount, AppState};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

use crate::jwt_auth::JwtMiddleware;

#[actix_web::delete("/avito/accounts/{id}")]
pub async fn delete_avito_account(
	path: web::Path<Uuid>,
	user: JwtMiddleware,
	data: web::Data<AppState>,
) -> Result<HttpResponse> {
	let mut conn = data.db.get().unwrap();
	let account_id = path.into_inner();

	// First, check if the account exists and belongs to the authenticated user
	let existing_account = match crate::schema::avito_accounts::table
		.find(account_id)
		.first::<AvitoAccount>(&mut conn)
	{
		Ok(account) => account,
		Err(diesel::result::Error::NotFound) => {
			return Ok(HttpResponse::NotFound().json(json!({
				"status": "fail",
				"message": "Avito account not found"
			})));
		}
		Err(e) => {
			eprintln!(
				"Database error when checking Avito account existence: {}",
				e
			);
			return Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to check Avito account existence"
			})));
		}
	};

	// Check if the account belongs to the authenticated user
	if existing_account.user_id != user.user_id {
		return Ok(HttpResponse::Forbidden().json(json!({
			"status": "fail",
			"message": "You don't have permission to delete this account"
		})));
	}

	match diesel::delete(crate::schema::avito_accounts::table.find(account_id)).execute(&mut conn) {
		Ok(rows_affected) => {
			if rows_affected > 0 {
				Ok(HttpResponse::Ok().json(json!({
					"status": "success",
					"message": "Avito account deleted successfully"
				})))
			} else {
				Ok(HttpResponse::NotFound().json(json!({
					"status": "fail",
					"message": "Avito account not found"
				})))
			}
		}
		Err(e) => {
			eprintln!("Database error when deleting Avito account: {}", e);
			Ok(HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": "Failed to delete Avito account"
			})))
		}
	}
}
