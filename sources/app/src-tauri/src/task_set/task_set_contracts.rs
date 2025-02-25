use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

use crate::placeholder::insert::placeholder_insert_contracts::PlaceholderInsertTileContract;

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSetContract {
	pub id: Uuid,
	pub project_id: Uuid,
	pub name: String,
	pub date_created: DateTime<Utc>,
	pub date_last_updated: DateTime<Utc>,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSetInfoContract {
	pub id: Uuid,
	pub project_id: Uuid,
	pub name: String,
	pub date_created: DateTime<Utc>,
	pub date_last_updated: DateTime<Utc>,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSetCreateContract {
	pub project_id: Uuid,
	pub name: String,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSetUpdateContract {
	pub id: Uuid,
	pub name: String,
}
