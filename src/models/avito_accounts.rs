use crate::schema::avito_accounts;
use chrono::{NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = avito_accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AvitoAccount {
	pub account_id: Uuid,
	pub user_id: Uuid,
	pub client_id: String,
	pub avito_client_secret: String,
	pub avito_client_id: String,
	pub is_connected: Option<bool>,
	pub created_ts: NaiveDateTime,
	pub updated_ts: NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = avito_accounts)]
pub struct CreateAvitoAccount {
	pub user_id: Uuid,
	pub client_id: String,
	pub avito_client_secret: String,
	pub avito_client_id: String,
	pub is_connected: Option<bool>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = avito_accounts)]
pub struct UpdateAvitoAccount {
	pub user_id: Option<Uuid>,
	pub client_id: Option<String>,
	pub avito_client_secret: Option<String>,
	pub avito_client_id: Option<String>,
	pub is_connected: Option<bool>,
	pub updated_ts: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub struct AvitoAccountResponse {
	pub status: String,
	pub data: AvitoAccountData,
}

#[derive(Serialize)]
pub struct AvitoAccountData {
	pub avito_account: AvitoAccount,
}

#[derive(Serialize)]
pub struct AvitoAccountsResponse {
	pub status: String,
	pub results: usize,
	pub data: AvitoAccountsData,
}

#[derive(Serialize)]
pub struct AvitoAccountsData {
	pub avito_accounts: Vec<AvitoAccount>,
}
