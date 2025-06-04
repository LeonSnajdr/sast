use crate::prelude::*;
use crate::task_set::session::task_set_session_contracts::TaskSetSessionContract;
use crate::task_set::session::task_set_session_filters::TaskSetSessionFilter;
use crate::task_set::session::task_set_session_service;

#[tauri::command]
#[specta::specta]
pub async fn task_set_session_get_many(filter: TaskSetSessionFilter) -> Result<Vec<TaskSetSessionContract>> {
	let task_set_sessions = task_set_session_service::get_many(&filter).await?;

	Ok(task_set_sessions)
}
