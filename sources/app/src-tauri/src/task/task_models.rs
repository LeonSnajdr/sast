use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct TaskModel {
	pub id: Uuid,
	pub project_id: Uuid,
	pub name: String,
	pub blocking: bool,
	pub date_created: DateTime<Utc>,
	pub date_last_updated: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct TaskInfoModel {
	pub id: Uuid,
	pub project_id: Uuid,
	pub name: String,
	pub blocking: bool,
	pub date_created: DateTime<Utc>,
	pub date_last_updated: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct TaskUpdateModel {
	pub id: Uuid,
	pub name: String,
	pub blocking: bool,
	pub date_last_updated: DateTime<Utc>,
}
