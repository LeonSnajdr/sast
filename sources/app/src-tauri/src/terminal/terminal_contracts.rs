use crate::terminal::terminal_enums::{TerminalHistoryPersistence, TerminalShellStatus};
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalSpawnContract {
	pub project_id: Uuid,
	pub task_id: Option<Uuid>,
	pub name: Option<String>,
	pub working_dir: Option<String>,
	pub command: Option<String>,
	pub no_exit: bool,
	pub force_kill: bool,
	pub history_persistence: TerminalHistoryPersistence,
}

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalInfoContract {
	pub id: Uuid,
	pub project_id: Uuid,
	pub task_id: Option<Uuid>,
	pub name: String,
	pub shell_status: TerminalShellStatus,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalResizeContract {
	pub cols: u16,
	pub rows: u16,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalFilterContract {
	pub id: Option<Uuid>,
	pub project_id: Option<Uuid>,
	pub task_id: Option<Uuid>,
}
impl Default for TerminalFilterContract {
	fn default() -> Self {
		Self {
			id: None,
			project_id: None,
			task_id: None,
		}
	}
}
