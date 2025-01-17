use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct ProjectModel {
	pub id: Uuid,
	pub name: String,
	pub date_created: DateTime<Utc>,
	pub date_last_opened: DateTime<Utc>,
}
