use tauri::AppHandle;
use uuid::Uuid;

use crate::prelude::*;
use crate::task_set::task_set_contracts::{TaskSetContract, TaskSetCreateContract, TaskSetInfoContract, TaskSetUpdateContract};
use crate::task_set::task_set_service;

#[tauri::command]
#[specta::specta]
pub async fn task_set_create(task_set_create_contract: TaskSetCreateContract) -> Result<Uuid> {
	let task_set_id = task_set_service::create(task_set_create_contract).await?;

	Ok(task_set_id)
}

#[tauri::command]
#[specta::specta]
pub async fn task_set_get_many_info(project_id: Uuid) -> Result<Vec<TaskSetInfoContract>> {
	let task_sets = task_set_service::get_many_info(project_id).await?;

	Ok(task_sets)
}

#[tauri::command]
#[specta::specta]
pub async fn task_set_get_one(id: Uuid) -> Result<TaskSetContract> {
	let task_set = task_set_service::get_one(id).await?;

	Ok(task_set)
}

#[tauri::command]
#[specta::specta]
pub async fn task_set_update_one(task_set_update_contract: TaskSetUpdateContract) -> Result<()> {
	task_set_service::update_one(task_set_update_contract).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn task_set_delete_one(id: Uuid) -> Result<()> {
	task_set_service::delete_one(id).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn task_set_start_one(app_handle: AppHandle, project_id: Uuid, task_set_id: Uuid) -> Result<()> {
	task_set_service::start_one(&app_handle, project_id, task_set_id).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn task_set_restart_one(app_handle: AppHandle, project_id: Uuid, task_set_id: Uuid) -> Result<()> {
	task_set_service::restart_one(&app_handle, project_id, task_set_id).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn task_set_stop_one(app_handle: AppHandle, task_set_id: Uuid) -> Result<()> {
	task_set_service::stop_one(&app_handle, task_set_id).await?;

	Ok(())
}
