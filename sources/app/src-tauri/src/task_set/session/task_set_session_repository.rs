use crate::prelude::*;
use crate::task_set::session::task_set_session_filters::TaskSetSessionFilter;
use crate::task_set::session::task_set_session_models::{TaskSetSessionInfoModel, TaskSetSessionModel, TaskSetSessionTaskInfoModel};
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

static STATE: Lazy<RwLock<Vec<Arc<TaskSetSessionModel>>>> = Lazy::new(|| RwLock::new(Vec::new()));

pub async fn create(task_set_session: TaskSetSessionModel) -> Result<Arc<TaskSetSessionModel>> {
	let session_arc = Arc::new(task_set_session);
	let session_return = Arc::clone(&session_arc);

	STATE.write().await.push(session_arc);

	Ok(session_return)
}

pub async fn get_one(id: &Uuid) -> Result<Arc<TaskSetSessionModel>> {
	let sessions = STATE.read().await;
	let session = sessions.iter().find(|&s| s.id == *id).ok_or(Error::NotExists)?.clone();

	Ok(session)
}

pub async fn get_many_info(filter: &TaskSetSessionFilter) -> Result<Vec<TaskSetSessionInfoModel>> {
	let sessions = STATE.read().await;

	let mut info_models = Vec::new();
	for session in sessions.iter().cloned() {
		if matches_filter(&session, filter).await {
			let info_model = to_info_model(&session).await;
			info_models.push(info_model);
		}
	}

	Ok(info_models)
}

pub async fn get_one_info(id: &Uuid) -> Result<TaskSetSessionInfoModel> {
	let session = get_one(id).await?;

	let info_model = to_info_model(&session).await;
	
	Ok(info_model)
}

async fn to_info_model(session: &Arc<TaskSetSessionModel>) -> TaskSetSessionInfoModel {
	let tasks = session.tasks.read().await;
	let mut task_infos = Vec::with_capacity(tasks.len());

	for task in tasks.iter() {
		task_infos.push(TaskSetSessionTaskInfoModel {
			task_id: task.task_id,
			task_name: task.task_name.clone(),
			date_started: task.date_started.read().await.clone(),
			date_finished: task.date_finished.read().await.clone(),
			status: task.status.read().await.clone(),
		});
	}

	TaskSetSessionInfoModel {
		id: session.id,
		project_id: session.project_id,
		task_set_id: session.task_set_id,
		kind: session.kind.clone(),
		date_started: session.date_started,
		date_finished: session.date_finished.read().await.clone(),
		status: session.status.read().await.clone(),
		tasks: task_infos,
	}
}

async fn matches_filter(terminal: &Arc<TaskSetSessionModel>, filter: &TaskSetSessionFilter) -> bool {
	let matches_id = filter.id.map_or(true, |id| terminal.id == id);
	let matches_project_id = filter.project_id.map_or(true, |id| terminal.project_id == id);
	let matches_task_set_id = filter.task_set_id.map_or(true, |id| terminal.task_set_id == id);

	matches_id && matches_project_id && matches_task_set_id
}
