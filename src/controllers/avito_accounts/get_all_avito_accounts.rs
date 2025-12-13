use crate::jwt_auth::JwtMiddleware;
use crate::{models::{AvitoAccount, AvitoAccountsResponse, AvitoAccountsData}, AppState};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;
use std::error::Error;
use uuid::Uuid;

use crate::utils::encryption;

#[derive(serde::Deserialize)]
pub struct Pagination {
    page: Option<i64>,
    limit: Option<i64>,
}

#[actix_web::get("/avito/accounts")]
pub async fn get_all_avito_accounts(
    user: JwtMiddleware,
    pagination: web::Query<Pagination>,
    data: web::Data<AppState>
) -> Result<HttpResponse> {
    let mut conn = data.db.get().unwrap();

    // Get pagination parameters
    let page = pagination.page.unwrap_or(1).max(1);
    let limit = pagination.limit.unwrap_or(10).min(10); // max 100 per page
    let offset = (page - 1) * limit;

    // Query avito accounts for the authenticated user with pagination
    let avito_accounts: Vec<AvitoAccount> = crate::schema::avito_accounts::table
        .filter(crate::schema::avito_accounts::user_id.eq(user.user_id)) // Proper UUID comparison
        .limit(limit)
        .offset(offset)
        .load(&mut conn)
        .map_err(|e| {
            eprintln!("Database error when fetching Avito accounts: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to fetch Avito accounts")
        })?;

    // Decrypt sensitive data before returning
    let mut decrypted_accounts: Vec<AvitoAccount> = Vec::new();
    for mut acc in avito_accounts {
        // Attempt to decrypt credentials
        match crate::utils::encryption::decrypt_avito_credentials(&acc.avito_client_secret, &acc.avito_client_id) {
            Ok((decrypted_secret, decrypted_client_id)) => {
                acc.avito_client_secret = decrypted_secret;
                acc.avito_client_id = decrypted_client_id;
            }
            Err(e) => {
                eprintln!("Failed to decrypt credentials for account {}: {}", acc.account_id, e);
                return Ok(HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": "Failed to decrypt account credentials"
                })));
            }
        }
        decrypted_accounts.push(acc);
    }

    Ok(HttpResponse::Ok().json(AvitoAccountsResponse {
        status: "success".to_string(),
        results: decrypted_accounts.len(),
        data: AvitoAccountsData {
            avito_accounts: decrypted_accounts
        },
    }))
}