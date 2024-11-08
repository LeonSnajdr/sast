use crate::db::types::UtcDateTime;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct ProjectModel {
	pub id: i64,
	pub name: String,
	pub date_created: UtcDateTime,
	pub date_last_opened: UtcDateTime,
}
