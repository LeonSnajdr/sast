use crate::task::task_contracts::TaskInfoContract;
use crate::terminal::shell::shell_contracts::ShellSpawnContract;
use crate::terminal::terminal_enums::{TerminalHistoryPersistence, TerminalShellStatus};
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalCreateContract {
	pub project_id: Uuid,
	pub task_id: Option<Uuid>,
	pub name: Option<String>,
	pub history_persistence: TerminalHistoryPersistence,
}

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalInfoContract {
	pub id: Uuid,
	pub project_id: Uuid,
	pub task: Option<TaskInfoContract>,
	pub name: String,
	pub shell_status: TerminalShellStatus,
}
