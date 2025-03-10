use crate::task_set::task::task_set_task_contracts::TaskSetTaskInfoContract;
use crate::task_set::task::task_set_task_models::{TaskSetTaskInfoModel, TaskSetTaskModel};
use uuid::Uuid;

impl TaskSetTaskInfoContract {
	pub fn from(value: TaskSetTaskInfoModel) -> Self {
		Self {
			task_id: value.task_id,
			task_name: value.task_name,
			task_date_created: value.task_date_created,
			task_date_last_updated: value.task_date_last_updated,
			blocking: value.blocking,
		}
	}
}

impl TaskSetTaskModel {
	pub fn from(id: Uuid, task_set_id: Uuid, position: i64, value: TaskSetTaskInfoContract) -> Self {
		Self {
			id,
			task_id: value.task_id,
			task_set_id,
			blocking: value.blocking,
			position,
		}
	}
}
