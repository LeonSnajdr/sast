use crate::task::task_contracts::TaskInfoContract;
use crate::terminal::terminal_contracts::{TerminalInfoContract, TerminalOpenContract};
use crate::terminal::terminal_models::{TerminalInfoModel, TerminalOpenModel};

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

impl TerminalOpenContract {
	pub fn from(value: TerminalOpenModel) -> Self {
		Self {
			history: value.history,
			shell_size: value.shell_size,
		}
	}
}
