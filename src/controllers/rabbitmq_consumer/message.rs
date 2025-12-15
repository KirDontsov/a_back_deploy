use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgressUpdateMessage {
	pub task_id: String,
	pub user_id: Uuid,
	pub request_id: Uuid,
	pub progress: f64,
	pub total_ads: i32,
	pub current_ads: i32,
	pub status: String,
	pub message: String,
	pub timestamp: String,
}

impl ProgressUpdateMessage {
	pub fn new(
		task_id: String,
		user_id: Uuid,
		request_id: Uuid,
		progress: f64,
		total_ads: i32,
		current_ads: i32,
		status: String,
		message: String,
		timestamp: String,
	) -> Self {
		Self {
			task_id,
			user_id,
			request_id,
			progress,
			total_ads,
			current_ads,
			status,
			message,
			timestamp,
		}
	}
}
