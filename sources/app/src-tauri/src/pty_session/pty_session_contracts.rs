use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PtySessionSpawnContract {
	pub project_id: Uuid,
	pub task_set_id: Option<Uuid>,
	pub name: Option<String>,
	pub working_dir: Option<String>,
	pub command: Option<String>,
	pub no_exit: bool
}

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PtySessionInfoContract {
	pub session_id: Uuid,
	pub project_id: Uuid,
	pub name: String,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PtySessionResizeContract {
	pub cols: u16,
	pub rows: u16,
}
