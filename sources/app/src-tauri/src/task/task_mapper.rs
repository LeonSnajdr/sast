use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::placeholder::insert::placeholder_insert_contracts::PlaceholderInsertTileContract;
use crate::task::task_contracts::{TaskContract, TaskCreateContract, TaskInfoContract, TaskUpdateContract};
use crate::task::task_models::{TaskInfoModel, TaskModel, TaskUpdateModel};

impl TaskContract {
	pub fn from(command_tiles: Vec<PlaceholderInsertTileContract>, working_dir_tiles: Vec<PlaceholderInsertTileContract>, value: TaskModel) -> Self {
		Self {
			id: value.id,
			project_id: value.project_id,
			name: value.name,
			tab_name: value.tab_name,
			no_exit: value.no_exit,
			force_kill: value.force_kill,
			history_persistence: value.history_persistence,
			command_tiles,
			working_dir_tiles,
			date_created: value.date_created,
			date_last_updated: value.date_last_updated,
		}
	}
}

impl TaskInfoContract {
	pub fn from(value: TaskInfoModel) -> Self {
		Self {
			id: value.id,
			project_id: value.project_id,
			name: value.name,
			date_created: value.date_created,
			date_last_updated: value.date_last_updated,
		}
	}
}

impl TaskModel {
	pub fn from_create_contract(
		id: Uuid,
		date_created: DateTime<Utc>,
		date_last_updated: DateTime<Utc>,
		value: TaskCreateContract,
	) -> (Self, Vec<PlaceholderInsertTileContract>, Vec<PlaceholderInsertTileContract>) {
		let TaskCreateContract {
			project_id,
			name,
			tab_name,
			no_exit,
			force_kill,
			history_persistence,
			command_tiles,
			working_dir_tiles,
		} = value;

		let task_create_model = Self {
			id,
			project_id,
			name,
			tab_name,
			no_exit,
			force_kill,
			history_persistence,
			date_created,
			date_last_updated,
		};

		(task_create_model, command_tiles, working_dir_tiles)
	}
}

impl TaskUpdateModel {
	pub fn from(date_last_updated: DateTime<Utc>, value: TaskUpdateContract) -> (Self, Vec<PlaceholderInsertTileContract>, Vec<PlaceholderInsertTileContract>) {
		let TaskUpdateContract {
			id,
			name,
			tab_name,
			no_exit,
			force_kill,
			history_persistence,
			command_tiles,
			working_dir_tiles,
		} = value;

		let task_update_model = Self {
			id,
			name,
			tab_name,
			no_exit,
			force_kill,
			history_persistence,
			date_last_updated,
		};

		(task_update_model, command_tiles, working_dir_tiles)
	}
}
