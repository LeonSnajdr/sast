use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct TaskSetModel {
	pub id: Uuid,
	pub project_id: Uuid,
	pub name: String,
	pub date_created: DateTime<Utc>,
	pub date_last_updated: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct TaskSetInfoModel {
	pub id: Uuid,
	pub project_id: Uuid,
	pub name: String,
	pub date_created: DateTime<Utc>,
	pub date_last_updated: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct TaskSetUpdateModel {
	pub id: Uuid,
	pub name: String,
	pub date_last_updated: DateTime<Utc>,
}
