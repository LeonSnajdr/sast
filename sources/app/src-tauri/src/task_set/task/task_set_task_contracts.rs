use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;
#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSetTaskInfoContract {
	pub task_id: Uuid,
	pub task_name: String,
	pub task_date_created: DateTime<Utc>,
	pub task_date_last_updated: DateTime<Utc>,
	pub blocking: bool,
	pub jump_into: bool,
}
