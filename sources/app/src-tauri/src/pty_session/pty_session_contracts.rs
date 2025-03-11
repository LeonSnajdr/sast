use crate::placeholder::insert::placeholder_insert_contracts::PlaceholderInsertTileFilterContract;
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PtySessionSpawnContract {
	pub project_id: Uuid,
	pub task_id: Option<Uuid>,
	pub task_set_id: Option<Uuid>,
	pub name: Option<String>,
	pub working_dir: Option<String>,
	pub command: Option<String>,
	pub no_exit: bool,
}

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PtySessionInfoContract {
	pub id: Uuid,
	pub project_id: Uuid,
	pub task_id: Option<Uuid>,
	pub task_set_id: Option<Uuid>,
	pub name: String,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PtySessionResizeContract {
	pub cols: u16,
	pub rows: u16,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PtySessionFilterContract {
	pub project_id: Option<Uuid>,
	pub task_id: Option<Uuid>,
	pub task_set_id: Option<Uuid>,
}
impl Default for PtySessionFilterContract {
	fn default() -> Self {
		Self {
			project_id: None,
			task_id: None,
			task_set_id: None,
		}
	}
}
