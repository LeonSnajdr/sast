use crate::task_set::task::task_set_task_contracts::TaskSetTaskInfoContract;
use crate::task_set::task_set_contracts::{TaskSetContract, TaskSetCreateContract, TaskSetInfoContract, TaskSetUpdateContract};
use crate::task_set::task_set_models::{TaskSetInfoModel, TaskSetModel, TaskSetUpdateModel};
use chrono::{DateTime, Utc};
use uuid::Uuid;

impl TaskSetContract {
	pub fn from(tasks: Vec<TaskSetTaskInfoContract>, value: TaskSetModel) -> Self {
		Self {
			id: value.id,
			project_id: value.project_id,
			name: value.name,
			date_created: value.date_created,
			date_last_updated: value.date_last_updated,
			tasks,
		}
	}
}

impl TaskSetInfoContract {
	pub fn from(value: TaskSetInfoModel) -> Self {
		Self {
			id: value.id,
			project_id: value.project_id,
			name: value.name,
			date_created: value.date_created,
			date_last_updated: value.date_last_updated,
		}
	}
}

impl TaskSetModel {
	pub fn from_create_contract(id: Uuid, date_created: DateTime<Utc>, date_last_updated: DateTime<Utc>, value: TaskSetCreateContract) -> Self {
		Self {
			id,
			project_id: value.project_id,
			name: value.name,
			date_created,
			date_last_updated,
		}
	}
}

impl TaskSetUpdateModel {
	pub fn from(date_last_updated: DateTime<Utc>, value: TaskSetUpdateContract) -> (Self, Vec<TaskSetTaskInfoContract>) {
		let TaskSetUpdateContract { id, name, tasks } = value;

		let update_model = Self { id, name, date_last_updated };

		(update_model, tasks)
	}
}
