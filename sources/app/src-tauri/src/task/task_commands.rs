use tauri::AppHandle;
use uuid::Uuid;

use crate::prelude::*;
use crate::task::task_contracts::{TaskContract, TaskCreateContract, TaskInfoContract, TaskUpdateContract};
use crate::task::task_service;

#[tauri::command]
#[specta::specta]
pub async fn task_create(task_create_contract: TaskCreateContract) -> Result<TaskContract> {
	let task_id = task_service::create(task_create_contract).await?;

	Ok(task_id)
}

#[tauri::command]
#[specta::specta]
pub async fn task_get_many_info(project_id: Uuid) -> Result<Vec<TaskInfoContract>> {
	let tasks = task_service::get_many_info(project_id).await?;

	Ok(tasks)
}

#[tauri::command]
#[specta::specta]
pub async fn task_get_one(id: Uuid) -> Result<TaskContract> {
	let task = task_service::get_one(id).await?;

	Ok(task)
}

#[tauri::command]
#[specta::specta]
pub async fn task_update_one(task_update_contract: TaskUpdateContract) -> Result<TaskContract> {
	let task = task_service::update_one(task_update_contract).await?;

	Ok(task)
}

#[tauri::command]
#[specta::specta]
pub async fn task_delete_one(id: Uuid) -> Result<()> {
	task_service::delete_one(id).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn task_start_one(app_handle: AppHandle, project_id: Uuid, task_id: Uuid) -> Result<()> {
	task_service::start_one(app_handle, project_id, task_id).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn task_restart_one(app_handle: AppHandle, project_id: Uuid, task_id: Uuid) -> Result<()> {
	task_service::restart_one(app_handle, project_id, task_id).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn task_stop_one(task_id: Uuid) -> Result<()> {
	task_service::stop_one(task_id).await?;

	Ok(())
}
