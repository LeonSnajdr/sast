use crate::prelude::*;
use crate::task_set::task_set_contracts::{TaskSetContract, TaskSetCreateContract, TaskSetInfoContract, TaskSetUpdateContract};
use crate::task_set::task_set_models::{TaskSetModel, TaskSetUpdateModel};
use crate::task_set::task_set_repository;
use chrono::Utc;
use tauri::AppHandle;
use uuid::Uuid;
use crate::task_set::task::task_set_task_service;

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

	let task_set_info_contracts = task_set_info_models.into_iter().map(TaskSetInfoContract::from).collect();

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
	Ok(())
}

pub async fn restart_one(app_handle: AppHandle, project_id: Uuid, task_set_id: Uuid) -> Result<()> {
	Ok(())
}

pub async fn stop_one(task_set_id: Uuid) -> Result<()> {
	Ok(())
}
