use crate::task_set::session::task_set_session_contracts::{TaskSetSessionContract, TaskSetSessionTaskContract};
use crate::task_set::session::task_set_session_models::{TaskSetSessionInfoModel, TaskSetSessionTaskInfoModel};

impl TaskSetSessionContract {
	pub fn from(value: TaskSetSessionInfoModel) -> Self {
		Self {
			id: value.id,
			project_id: value.project_id,
			task_set_id: value.task_set_id,
			kind: value.kind,
			date_started: value.date_started,
			date_finished: value.date_finished,
			status: value.status,
			tasks: value.tasks.into_iter().map(TaskSetSessionTaskContract::from).collect(),
		}
	}
}

impl TaskSetSessionTaskContract {
	pub fn from(value: TaskSetSessionTaskInfoModel) -> Self {
		Self {
			task_id: value.task_id,
			task_name: value.task_name,
			date_started: value.date_started,
			date_finished: value.date_finished,
			status: value.status,
		}
	}
}
