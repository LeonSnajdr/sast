use crate::terminal::terminal_contracts::{TerminalFilterContract, TerminalInfoContract};
use crate::terminal::terminal_models::{TerminalFilterModel, TerminalInfoModel};

impl TerminalInfoContract {
	pub fn from(value: TerminalInfoModel) -> Self {
		Self {
			id: value.id,
			name: value.name,
			project_id: value.project_id,
			task_id: value.task_id,
			task_set_id: value.task_set_id,
			shell_status: value.shell_status,
		}
	}
}

impl TerminalFilterModel {
	pub fn from(value: TerminalFilterContract) -> Self {
		Self {
			id: value.id,
			project_id: value.project_id,
			task_id: value.task_id,
			task_set_id: value.task_set_id,
		}
	}
}
