use crate::schema::users;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
	pub id: Uuid,
	pub name: Option<String>,
	pub email: String,
	pub password: String,
	pub role: Option<String>,
	pub photo: Option<String>,
	pub verified: Option<bool>,
	pub favourite: Vec<Option<String>>,
	pub created_at: Option<NaiveDateTime>,
	pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct CreateUser {
	pub name: Option<String>,
	pub email: String,
	pub password: String,
	pub role: Option<String>,
	pub photo: Option<String>,
	pub verified: Option<bool>,
	pub favourite: Vec<Option<String>>,
	pub created_at: Option<chrono::NaiveDateTime>,
	pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct UpdateUser {
	pub name: Option<String>,
	pub email: Option<String>,
	pub role: Option<String>,
	pub photo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
	pub sub: String,
	pub exp: usize,
	pub iat: usize,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
	pub email: String,
	pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
	pub name: Option<String>,
	pub email: String,
	pub password: String,
	pub password_confirm: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
	pub status: String,
	pub token: String,
}

#[derive(Serialize)]
pub struct UserResponse {
	pub status: String,
	pub data: UserData,
}

#[derive(Serialize)]
pub struct UserData {
	pub user: User,
}
