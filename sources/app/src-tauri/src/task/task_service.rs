use crate::placeholder::insert::placeholder_insert_contracts::PlaceholderInsertTileFilterContract;
use crate::placeholder::insert::placeholder_insert_service;
use crate::prelude::*;
use crate::pty_session::pty_session_contracts::{PtySessionFilterContract, PtySessionSpawnContract};
use crate::pty_session::pty_session_service;
use crate::task::task_contracts::{TaskContract, TaskCreateContract, TaskInfoContract, TaskUpdateContract};
use crate::task::task_models::{TaskModel, TaskUpdateModel};
use crate::task::task_repository;
use chrono::Utc;
use tauri::AppHandle;
use uuid::Uuid;

pub async fn create(task_create_contract: TaskCreateContract) -> Result<TaskContract> {
	let id = Uuid::new_v4();
	let date_created = Utc::now();
	let date_last_updated = Utc::now();

	let (task_create_model, command_tiles, working_dir_tiles) = TaskModel::from_create_contract(id, date_created, date_last_updated, task_create_contract);

	task_repository::create(task_create_model).await?;

	let delete_command_tiles_filter = PlaceholderInsertTileFilterContract {
		task_command_id: Some(id),
		..PlaceholderInsertTileFilterContract::default()
	};

	placeholder_insert_service::create_or_replace(command_tiles, delete_command_tiles_filter).await?;

	let delete_working_dir_tiles_filter = PlaceholderInsertTileFilterContract {
		task_working_dir_id: Some(id),
		..PlaceholderInsertTileFilterContract::default()
	};

	placeholder_insert_service::create_or_replace(working_dir_tiles, delete_working_dir_tiles_filter).await?;

	let task = get_one(id).await?;

	Ok(task)
}

pub async fn get_one(id: Uuid) -> Result<TaskContract> {
	let task_model = task_repository::get_one(id).await?;

	let command_tile_filter = PlaceholderInsertTileFilterContract {
		task_command_id: Some(id),
		..PlaceholderInsertTileFilterContract::default()
	};

	let command_tile_contracts = placeholder_insert_service::get_many(command_tile_filter).await?;

	let working_dir_tile_filter = PlaceholderInsertTileFilterContract {
		task_working_dir_id: Some(id),
		..PlaceholderInsertTileFilterContract::default()
	};

	let working_dir_tile_contracts = placeholder_insert_service::get_many(working_dir_tile_filter).await?;

	let task_contract = TaskContract::from(command_tile_contracts, working_dir_tile_contracts, task_model);

	Ok(task_contract)
}

pub async fn get_many_info(project_id: Uuid) -> Result<Vec<TaskInfoContract>> {
	let task_info_models = task_repository::get_many_info(project_id).await?;

	let task_info_contracts = task_info_models.into_iter().map(TaskInfoContract::from).collect();

	Ok(task_info_contracts)
}

pub async fn update_one(task_update_contract: TaskUpdateContract) -> Result<TaskContract> {
	let task_set_id = task_update_contract.id;

	let date_last_updated = Utc::now();

	let (task_update_model, command_tiles, working_dir_tiles) = TaskUpdateModel::from(date_last_updated, task_update_contract);

	let delete_command_tiles_filter = PlaceholderInsertTileFilterContract {
		task_command_id: Some(task_set_id),
		..PlaceholderInsertTileFilterContract::default()
	};

	placeholder_insert_service::create_or_replace(command_tiles, delete_command_tiles_filter).await?;

	let delete_working_dir_tiles_filter = PlaceholderInsertTileFilterContract {
		task_working_dir_id: Some(task_set_id),
		..PlaceholderInsertTileFilterContract::default()
	};

	placeholder_insert_service::create_or_replace(working_dir_tiles, delete_working_dir_tiles_filter).await?;

	task_repository::update_one(task_update_model).await?;

	let task = get_one(task_set_id).await?;

	Ok(task)
}

pub async fn delete_one(id: Uuid) -> Result<()> {
	task_repository::delete_one(id).await?;

	Ok(())
}

pub async fn start_one(app_handle: &AppHandle, project_id: Uuid, task_id: Uuid) -> Result<()> {
	let pty_spawn_contract = build_spawn_contract(project_id, task_id).await?;

	pty_session_service::spawn(app_handle, pty_spawn_contract).await?;

	Ok(())
}

pub async fn build_spawn_contract(project_id: Uuid, task_id: Uuid) -> Result<PtySessionSpawnContract> {
	let command_tiles_filter = PlaceholderInsertTileFilterContract {
		task_command_id: Some(task_id),
		..PlaceholderInsertTileFilterContract::default()
	};

	let working_dir_tiles_filter = PlaceholderInsertTileFilterContract {
		task_working_dir_id: Some(task_id),
		..PlaceholderInsertTileFilterContract::default()
	};

	let task = task_repository::get_one(task_id).await?;
	let command = placeholder_insert_service::get_rendered_tiles(command_tiles_filter).await?;
	let working_dir = placeholder_insert_service::get_rendered_tiles(working_dir_tiles_filter).await?;

	let pty_spawn_contract = PtySessionSpawnContract {
		project_id,
		task_id: Some(task_id),
		task_set_id: None,
		name: task.tab_name,
		command,
		working_dir,
		no_exit: task.no_exit,
		force_kill: task.force_kill,
		history_persistence: task.history_persistence,
	};

	Ok(pty_spawn_contract)
}

pub async fn restart_one(app_handle: &AppHandle, project_id: Uuid, task_id: Uuid) -> Result<()> {
	/*stop_one(task_id).await?;
	start_one(app_handle, project_id, task_id).await?;*/

	let filter = PtySessionFilterContract {
		task_id: Some(task_id),
		..PtySessionFilterContract::default()
	};

	pty_session_service::restart_first(filter).await?;

	Ok(())
}

pub async fn stop_one(task_id: Uuid) -> Result<()> {
	let filter = PtySessionFilterContract {
		task_id: Some(task_id),
		..PtySessionFilterContract::default()
	};

	pty_session_service::kill_many(filter).await?;

	Ok(())
}
