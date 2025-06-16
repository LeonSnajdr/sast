use chrono::Utc;
use std::sync::Arc;
use tauri::AppHandle;
use tauri_specta::Event;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::prelude::*;
use crate::task_set::session::task_set_session_contracts::TaskSetSessionContract;
use crate::task_set::session::task_set_session_enums::{TaskSetSessionKind, TaskSetSessionStatus, TaskSetSessionTaskStatus};
use crate::task_set::session::task_set_session_events::{TaskSetSessionStartedEvent, TaskSetSessionUpdatedEvent};
use crate::task_set::session::task_set_session_filters::TaskSetSessionFilter;
use crate::task_set::session::task_set_session_models::{TaskSetSessionModel, TaskSetSessionTaskModel};
use crate::task_set::session::task_set_session_repository;
use crate::task_set::task::task_set_task_models::TaskSetTaskInfoModel;

pub async fn get_many(filter: &TaskSetSessionFilter) -> Result<Vec<TaskSetSessionContract>> {
	let sessions = task_set_session_repository::get_many_info(filter).await?;

	let contracts = sessions.into_iter().map(|session| TaskSetSessionContract::from(session)).collect();

	Ok(contracts)
}

pub async fn get_one(id: &Uuid) -> Result<TaskSetSessionContract> {
	let session = task_set_session_repository::get_one_info(id).await?;

	let contract = TaskSetSessionContract::from(session);

	Ok(contract)
}

pub async fn start(
	app_handle: &AppHandle,
	project_id: &Uuid,
	task_set_id: &Uuid,
	task_set_tasks: &Vec<TaskSetTaskInfoModel>,
	kind: TaskSetSessionKind,
) -> Result<Uuid> {
	let project_id = project_id.clone();
	let task_set_id = task_set_id.clone();
	let task_set_tasks = task_set_tasks.clone();

	let session = build_session(project_id, task_set_id, task_set_tasks, kind);

	let created_session = task_set_session_repository::create(session).await?;

	let _ = TaskSetSessionStartedEvent(created_session.id).emit(app_handle);

	Ok(created_session.id)
}

pub async fn finish(app_handle: &AppHandle, session_id: &Uuid) -> Result<()> {
	let session = task_set_session_repository::get_one(session_id).await?;

	let mut status = TaskSetSessionStatus::Completed;
	for task_session in session.tasks.write().await.iter_mut() {
		let mut task_session_status = task_session.status.write().await;
		let mut task_session_date_finished = task_session.date_finished.write().await;

		match *task_session_status {
			TaskSetSessionTaskStatus::NotStarted => {
				*task_session_status = TaskSetSessionTaskStatus::Skipped;
				*task_session_date_finished = Some(Utc::now());
			}
			TaskSetSessionTaskStatus::Failed => status = TaskSetSessionStatus::Failed,
			_ => {}
		}
	}

	*session.status.write().await = status;
	*session.date_finished.write().await = Some(Utc::now());

	let _ = TaskSetSessionStartedEvent(session_id.clone()).emit(app_handle);

	Ok(())
}

fn build_session(project_id: Uuid, task_set_id: Uuid, task_set_tasks: Vec<TaskSetTaskInfoModel>, kind: TaskSetSessionKind) -> TaskSetSessionModel {
	let tasks = build_session_tasks(task_set_tasks);

	TaskSetSessionModel {
		id: Uuid::new_v4(),
		project_id,
		kind,
		date_started: Utc::now(),
		date_finished: Arc::new(RwLock::new(None)),
		task_set_id,
		status: Arc::new(RwLock::new(TaskSetSessionStatus::Running)),
		tasks: Arc::new(RwLock::new(tasks)),
	}
}

fn build_session_tasks(task_set_tasks: Vec<TaskSetTaskInfoModel>) -> Vec<TaskSetSessionTaskModel> {
	task_set_tasks
		.into_iter()
		.map(|task| TaskSetSessionTaskModel {
			task_id: task.task_id,
			task_name: task.task_name,
			date_started: Arc::new(RwLock::new(None)),
			date_finished: Arc::new(RwLock::new(None)),
			status: Arc::new(RwLock::new(TaskSetSessionTaskStatus::NotStarted)),
		})
		.collect()
}

pub async fn start_task(app_handle: &AppHandle, task_set_session_id: &Uuid, task_id: &Uuid) -> Result<()> {
	let session = task_set_session_repository::get_one(task_set_session_id).await?;

	let mut tasks_guard = session.tasks.write().await;
	if let Some(task) = tasks_guard.iter_mut().find(|t| t.task_id == *task_id) {
		*task.date_started.write().await = Some(Utc::now());
		*task.status.write().await = TaskSetSessionTaskStatus::Running;

		let _ = TaskSetSessionUpdatedEvent(task_set_session_id.clone()).emit(app_handle);
	}

	Ok(())
}

pub async fn finish_task(app_handle: &AppHandle, task_set_session_id: &Uuid, task_id: &Uuid, status: TaskSetSessionTaskStatus) -> Result<()> {
	let session = task_set_session_repository::get_one(task_set_session_id).await?;

	let mut tasks_guard = session.tasks.write().await;
	if let Some(task) = tasks_guard.iter_mut().find(|t| t.task_id == *task_id) {
		*task.date_finished.write().await = Some(Utc::now());
		*task.status.write().await = status.clone();

		let _ = TaskSetSessionUpdatedEvent(task_set_session_id.clone()).emit(app_handle);
	}

	Ok(())
}
