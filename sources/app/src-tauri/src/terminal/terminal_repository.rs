use crate::error::Error;
use crate::prelude::*;
use crate::terminal::terminal_models::{TerminalFilterModel, TerminalInfoModel, TerminalModel};
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

static PTY_STATE: Lazy<PtyState> = Lazy::new(|| PtyState {
	sessions: RwLock::new(Vec::new()),
});

struct PtyState {
	sessions: RwLock<Vec<Arc<TerminalModel>>>,
}

pub async fn create_one(session: TerminalModel) -> Result<()> {
	PTY_STATE.sessions.write().await.push(Arc::new(session));
	Ok(())
}

pub async fn get_one(id: &Uuid) -> Result<Arc<TerminalModel>> {
	let sessions = PTY_STATE.sessions.read().await;
	let session = sessions.iter().find(|&s| s.id == *id).ok_or(Error::NotExists)?.clone();
	Ok(session)
}

pub async fn get_first(filter: TerminalFilterModel) -> Option<Arc<TerminalModel>> {
	let sessions = PTY_STATE.sessions.read().await;
	for session in sessions.iter().cloned() {
		if matches_filter(&session, &filter).await {
			return Some(session);
		}
	}
	None
}

pub async fn get_is_existing(filter: TerminalFilterModel) -> bool {
	let sessions = PTY_STATE.sessions.read().await;
	for session in sessions.iter().cloned() {
		if matches_filter(&session, &filter).await {
			return true;
		}
	}

	false
}

pub async fn get_many(filter: TerminalFilterModel) -> Result<Vec<Arc<TerminalModel>>> {
	let sessions = PTY_STATE.sessions.read().await;
	let mut filtered_sessions = Vec::new();
	for session in sessions.iter().cloned() {
		if matches_filter(&session, &filter).await {
			filtered_sessions.push(session);
		}
	}
	Ok(filtered_sessions)
}

pub async fn get_many_info(filter: TerminalFilterModel) -> Result<Vec<TerminalInfoModel>> {
	let filtered_sessions = get_many(filter).await?;
	let mut info_models = Vec::with_capacity(filtered_sessions.len());

	for session in filtered_sessions {
		let shell_guard = session.shell.read().await;
		let meta_guard = session.meta.read().await;

		info_models.push(TerminalInfoModel {
			id: session.id,
			project_id: meta_guard.project_id,
			name: meta_guard.name.clone(),
			task_id: meta_guard.task_id,
			shell_status: shell_guard.status.lock().await.clone(),
		});
	}

	Ok(info_models)
}

async fn matches_filter(session: &Arc<TerminalModel>, filter: &TerminalFilterModel) -> bool {
	let meta_guard = session.meta.read().await;

	let matches_id = filter.id.map_or(true, |id| session.id == id);
	let matches_project_id = filter.project_id.map_or(true, |id| meta_guard.project_id == id);
	let matches_task_id = filter.task_id.map_or(true, |id| meta_guard.task_id == Some(id));

	matches_id && matches_project_id && matches_task_id
}

pub async fn delete_one(id: &Uuid) -> Result<()> {
	let mut sessions = PTY_STATE.sessions.write().await;
	if let Some(pos) = sessions.iter().position(|s| s.id == *id) {
		sessions.remove(pos);
	}
	Ok(())
}
