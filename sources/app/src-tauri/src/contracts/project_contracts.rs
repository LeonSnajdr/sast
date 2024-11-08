use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Type, Serialize, Deserialize)]
pub struct CreateProjectContract {
	pub name: String,
}

#[derive(Debug, Type, Serialize, Deserialize)]
pub struct ProjectContract {
	pub id: i32,
	pub name: String,
	pub date_created: DateTime<Utc>,
	pub date_last_opened: DateTime<Utc>,
}
