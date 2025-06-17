use crate::task::task_contracts::TaskInfoContract;
use crate::task_set::task_set_contracts::TaskSetInfoContract;
use crate::terminal::shell::shell_contracts::ShellSizeContract;
use crate::terminal::terminal_enums::{TerminalHistoryPersistence, TerminalShellStatus};
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalCreateContract {
	pub project_id: Uuid,
	pub task_id: Option<Uuid>,
	pub task_set_id: Option<Uuid>,
	pub name: Option<String>,
	pub history_persistence: TerminalHistoryPersistence,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalOpenContract {
	pub history: String,
	pub shell_size: ShellSizeContract,
}

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalRestartContract {
	pub name: Option<String>,
	pub history_persistence: TerminalHistoryPersistence,
	pub task_set_id: Option<Uuid>,
}

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalInfoContract {
	pub id: Uuid,
	pub project_id: Uuid,
	pub task: Option<TaskInfoContract>,
	pub task_set: Option<TaskSetInfoContract>,
	pub name: String,
	pub shell_status: TerminalShellStatus,
}
