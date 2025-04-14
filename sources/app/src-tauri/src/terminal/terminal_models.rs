use crate::terminal::shell::shell_contracts::ShellSizeContract;
use crate::terminal::terminal_enums::TerminalShellStatus;
use uuid::Uuid;

pub struct TerminalOpenModel {
	pub history: String,
	pub shell_size: ShellSizeContract,
}

pub struct TerminalInfoModel {
	pub id: Uuid,
	pub project_id: Uuid,
	pub task_id: Option<Uuid>,
	pub name: String,
	pub shell_status: TerminalShellStatus,
}
