use crate::terminal::terminal_enums::TerminalShellStatus;
use uuid::Uuid;

pub struct TerminalInfoModel {
	pub id: Uuid,
	pub project_id: Uuid,
	pub task_id: Option<Uuid>,
	pub name: String,
	pub shell_status: TerminalShellStatus,
}
