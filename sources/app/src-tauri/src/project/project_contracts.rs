use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectContract {
	pub name: String,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectContract {
	pub id: Uuid,
	pub name: String,
	pub date_created: DateTime<Utc>,
	pub date_last_opened: DateTime<Utc>,
}
