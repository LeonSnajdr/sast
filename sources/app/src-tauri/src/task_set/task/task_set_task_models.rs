use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct TaskSetTaskModel {
	pub id: Uuid,
	pub task_id: Uuid,
	pub task_set_id: Uuid,
	pub blocking: bool,
	pub position: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct TaskSetTaskInfoModel {
	pub task_id: Uuid,
	pub task_name: String,
	pub task_date_created: DateTime<Utc>,
	pub task_date_last_updated: DateTime<Utc>,
	pub task_set_id: Uuid,
	pub blocking: bool,
}
