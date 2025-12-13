use crate::{models::{UpdateAvitoAccount, AvitoAccountResponse, AvitoAccountData}, AppState};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

use crate::utils::encryption;
use crate::models::AvitoAccount;
use crate::jwt_auth::JwtMiddleware;

#[actix_web::put("/avito/accounts/{id}")]
pub async fn update_avito_account(
    path: web::Path<Uuid>,
    user: JwtMiddleware,
    body: web::Json<UpdateAvitoAccount>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let mut conn = data.db.get().unwrap();
    let account_id = path.into_inner();

    // First, get the existing account to handle partial updates
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
            eprintln!("Database error when fetching existing Avito account: {}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to fetch existing Avito account"
            })));
        }
    };

    // Check if the account belongs to the authenticated user
    if existing_account.user_id != user.user_id {
        return Ok(HttpResponse::Forbidden().json(json!({
            "status": "fail",
            "message": "You don't have permission to update this account"
        })));
    }

    // Prepare update values, using existing values if not provided in the request
    let update_data = UpdateAvitoAccount {
        user_id: body.user_id.clone().or(Some(existing_account.user_id)),
        client_id: body.client_id.clone().or(Some(existing_account.client_id)),
        avito_client_secret: match encrypt_if_provided(&body.avito_client_secret, &existing_account.avito_client_secret) {
            Ok(value) => value,
            Err(response) => return Ok(response),
        },
        avito_client_id: match encrypt_if_provided(&body.avito_client_id, &existing_account.avito_client_id) {
            Ok(value) => value,
            Err(response) => return Ok(response),
        },
        is_connected: body.is_connected.or(existing_account.is_connected),
        updated_ts: Some(chrono::Utc::now().naive_utc()),
    };

    match diesel::update(crate::schema::avito_accounts::table.find(account_id))
        .set(&update_data)
        .get_result::<AvitoAccount>(&mut conn)
    {
        Ok(mut avito_account) => {
            // Decrypt credentials for the response
            match crate::utils::encryption::decrypt_avito_credentials(&avito_account.avito_client_secret, &avito_account.avito_client_id) {
                Ok((decrypted_secret, decrypted_client_id)) => {
                    avito_account.avito_client_secret = decrypted_secret;
                    avito_account.avito_client_id = decrypted_client_id;
                }
                Err(e) => {
                    eprintln!("Failed to decrypt credentials after update: {}", e);
                    return Ok(HttpResponse::InternalServerError().json(json!({
                        "status": "error",
                        "message": "Failed to decrypt account credentials"
                    })));
                }
            }

            Ok(HttpResponse::Ok().json(AvitoAccountResponse {
                status: "success".to_string(),
                data: AvitoAccountData {
                    avito_account
                },
            }))
        },
        Err(e) => {
            eprintln!("Database error when updating Avito account: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to update Avito account"
            })))
        }
    }
}

// Helper function to encrypt a field if provided, otherwise return existing encrypted value
fn encrypt_if_provided(new_value: &Option<String>, existing_encrypted: &str) -> Result<Option<String>, HttpResponse> {
    if let Some(ref value) = new_value {
        if value.is_empty() {
            return Err(HttpResponse::BadRequest().json(json!({
                "status": "error",
                "message": "Avito client secret and client ID cannot be empty"
            })));
        }
        match encrypt_field(value) {
            Ok(encrypted) => Ok(Some(encrypted)),
            Err(e) => {
                eprintln!("Failed to encrypt field: {}", e);
                Err(HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": "Failed to encrypt sensitive data"
                })))
            }
        }
    } else {
        Ok(Some(existing_encrypted.to_string()))
    }
}

// Helper function to encrypt a field
fn encrypt_field(field: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Global key for encryption (in production, this should be stored securely)
    static ENCRYPTION_KEY: [u8; 32] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
        27, 28, 29, 30, 31, 32,
    ];

    let iv = crate::utils::encryption::generate_iv();
    let encrypted_data = crate::utils::encryption::encrypt_data(field, &ENCRYPTION_KEY, &iv);
    Ok(crate::utils::encryption::combine_iv_and_data(&iv, &encrypted_data))
}