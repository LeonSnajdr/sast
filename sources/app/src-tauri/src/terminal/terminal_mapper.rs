use crate::task::task_contracts::TaskInfoContract;
use crate::terminal::terminal_contracts::TerminalInfoContract;
use crate::terminal::terminal_models::TerminalInfoModel;

impl TerminalInfoContract {
	pub fn from(value: TerminalInfoModel, task: Option<TaskInfoContract>) -> Self {
		Self {
			id: value.id,
			name: value.name,
			project_id: value.project_id,
			task,
			shell_status: value.shell_status,
		}
	}
}
