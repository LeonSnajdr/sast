use crate::db::types::{Key, UtcDateTime};

use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct ProjectModel {
	pub id: Key,
	pub name: String,
	pub date_created: UtcDateTime,
	pub date_last_opened: UtcDateTime,
}
