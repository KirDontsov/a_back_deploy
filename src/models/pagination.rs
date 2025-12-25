use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PaginationParams {
	pub page: Option<u32>,
	pub limit: Option<u32>,
}

#[derive(Serialize)]
pub struct PaginationResponse {
	pub page: u32,
	pub limit: u32,
	pub total: i64,
	pub pages: u32,
}

#[derive(Serialize)]
pub struct ResponseWithPagination<T> {
	pub status: String,
	pub data: T,
	pub pagination: PaginationResponse,
}
