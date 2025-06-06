use crate::prelude::*;
use crate::task_set::session::task_set_session_enums::{TaskSetSessionKind, TaskSetSessionTaskStatus};
use crate::task_set::session::task_set_session_service;
use crate::task_set::task::{task_set_task_repository, task_set_task_service};
use crate::task_set::task_set_contracts::{TaskSetContract, TaskSetCreateContract, TaskSetInfoContract, TaskSetUpdateContract};
use crate::task_set::task_set_models::{TaskSetModel, TaskSetUpdateModel};
use crate::task_set::task_set_repository;
use crate::terminal::terminal_enums::TerminalShellStatus;
use crate::terminal::terminal_filters::TerminalFilter;
use crate::terminal::terminal_service;
use chrono::Utc;
use tauri::AppHandle;
use uuid::Uuid;

pub async fn create(task_set_create_contract: TaskSetCreateContract) -> Result<Uuid> {
	let id = Uuid::new_v4();
	let date_created = Utc::now();
	let date_last_updated = Utc::now();

	let task_set_create_model = TaskSetModel::from_create_contract(id, date_created, date_last_updated, task_set_create_contract);

	task_set_repository::create(task_set_create_model).await?;

	Ok(id)
}

pub async fn get_one(id: Uuid) -> Result<TaskSetContract> {
	let task_set_model = task_set_repository::get_one(id).await?;

	let tasks = task_set_task_service::get_all_info(id).await?;

	let task_set_contract = TaskSetContract::from(tasks, task_set_model);

	Ok(task_set_contract)
}

pub async fn get_many_info(project_id: Uuid) -> Result<Vec<TaskSetInfoContract>> {
	let task_set_info_models = task_set_repository::get_many_info(project_id).await?;

	let mut task_set_info_contracts = Vec::new();

	for task_set_info_model in task_set_info_models {
		let task_ids = task_set_task_repository::get_all_task_ids(task_set_info_model.id).await?;

		task_set_info_contracts.push(TaskSetInfoContract::from(task_set_info_model, task_ids));
	}

	Ok(task_set_info_contracts)
}

pub async fn update_one(task_set_update_contract: TaskSetUpdateContract) -> Result<()> {
	let date_last_updated = Utc::now();

	let (task_set_update_model, tasks) = TaskSetUpdateModel::from(date_last_updated, task_set_update_contract);

	task_set_task_service::create_or_replace(task_set_update_model.id, tasks).await?;

	task_set_repository::update_one(task_set_update_model).await?;

	Ok(())
}

pub async fn delete_one(id: Uuid) -> Result<()> {
	task_set_repository::delete_one(id).await?;

	Ok(())
}

pub async fn start_one(app_handle: AppHandle, project_id: Uuid, task_set_id: Uuid) -> Result<()> {
	let task_set_task_infos = task_set_task_repository::get_all_info(task_set_id).await?;

	let session_id = task_set_session_service::start(&app_handle, &project_id, &task_set_id, &task_set_task_infos, TaskSetSessionKind::Start).await?;

	for task_set_task_info in task_set_task_infos {
		task_set_session_service::start_task(&app_handle, &session_id, &task_set_task_info.task_id).await?;

		let app_handle_clone = app_handle.clone();
		let create_contract = task_set_task_service::build_terminal_create_contract(project_id, &task_set_task_info).await?;
		let spawn_contract = task_set_task_service::build_shell_spawn_contract(&task_set_task_info).await?;

		let successful = if task_set_task_info.blocking {
			terminal_service::create_blocking(app_handle_clone, create_contract, spawn_contract).await?
		} else {
			terminal_service::create(app_handle_clone, create_contract, Some(spawn_contract))
				.await
				.map(|_| true)?
		};

		if !successful {
			task_set_session_service::finish_task(&app_handle, &session_id, &task_set_task_info.task_id, TaskSetSessionTaskStatus::Failed).await?;
			break;
		}

		task_set_session_service::finish_task(&app_handle, &session_id, &task_set_task_info.task_id, TaskSetSessionTaskStatus::Completed).await?;
	}

	task_set_session_service::finish(&app_handle, &session_id).await?;

	Ok(())
}

pub async fn restart_one(app_handle: AppHandle, project_id: Uuid, task_set_id: Uuid) -> Result<()> {
	let task_set_task_infos = task_set_task_repository::get_all_info(task_set_id).await?;

	let task_ids = task_set_task_infos.iter().map(|task_set_task| task_set_task.task_id).collect::<Vec<Uuid>>();

	let filter = TerminalFilter {
		task_ids: Some(task_ids),
		..TerminalFilter::default()
	};

	terminal_service::restart_schedule(&filter).await?;

	let session_id = task_set_session_service::start(&app_handle, &project_id, &task_set_id, &task_set_task_infos, TaskSetSessionKind::Restart).await?;

	for task_set_task_info in task_set_task_infos.iter() {
		task_set_session_service::start_task(&app_handle, &session_id, &task_set_task_info.task_id).await?;

		let filter = TerminalFilter {
			task_ids: Some(vec![task_set_task_info.task_id]),
			..TerminalFilter::default()
		};

		let is_terminal_existing = terminal_service::get_is_existing(&filter).await;

		let app_handle_clone = app_handle.clone();
		let create_contract = task_set_task_service::build_terminal_create_contract(project_id, &task_set_task_info).await?;
		let spawn_contract = task_set_task_service::build_shell_spawn_contract(&task_set_task_info).await?;

		let successful = match (is_terminal_existing, task_set_task_info.blocking) {
			(true, false) => terminal_service::shell_restart_first(&filter, spawn_contract).await.map(|_| true)?,
			(true, true) => terminal_service::shell_restart_first_blocking(&filter, spawn_contract).await?,
			(false, false) => terminal_service::create(app_handle_clone, create_contract, Some(spawn_contract))
				.await
				.map(|_| true)?,
			(false, true) => terminal_service::create_blocking(app_handle_clone, create_contract, spawn_contract).await?,
		};

		if !successful {
			task_set_session_service::finish_task(&app_handle, &session_id, &task_set_task_info.task_id, TaskSetSessionTaskStatus::Failed).await?;
			break;
		}

		task_set_session_service::finish_task(&app_handle, &session_id, &task_set_task_info.task_id, TaskSetSessionTaskStatus::Completed).await?;
	}

	let restarting_filter = TerminalFilter {
		shell_status: Some(vec![TerminalShellStatus::Restarting]),
		..filter
	};

	terminal_service::close_many(&restarting_filter).await?;
	task_set_session_service::finish(&app_handle, &session_id).await?;

	Ok(())
}

pub async fn stop_one(task_set_id: Uuid) -> Result<()> {
	let task_set_tasks = task_set_task_repository::get_all(task_set_id).await?;

	for task_set_task in task_set_tasks {
		let filter = TerminalFilter {
			task_ids: Some(vec![task_set_task.task_id]),
			..TerminalFilter::default()
		};

		terminal_service::close_first(&filter).await?
	}

	Ok(())
}
