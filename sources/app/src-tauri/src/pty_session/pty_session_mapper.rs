use crate::pty_session::pty_session_contracts::{PtySessionFilterContract, PtySessionInfoContract};
use crate::pty_session::pty_session_models::{PtySessionFilterModel, PtySessionInfoModel};

impl PtySessionInfoContract {
	pub fn from(value: PtySessionInfoModel) -> Self {
		Self {
			id: value.id,
			name: value.name,
			project_id: value.project_id,
			task_id: value.task_id,
			task_set_id: value.task_set_id,
		}
	}
}

impl PtySessionFilterModel {
	pub fn from(value: PtySessionFilterContract) -> Self {
		Self {
			project_id: value.project_id,
			task_id: value.task_id,
			task_set_id: value.task_set_id,
		}
	}
}
