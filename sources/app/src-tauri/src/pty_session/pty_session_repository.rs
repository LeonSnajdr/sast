use crate::error::Error;
use crate::prelude::*;
use crate::pty_session::pty_session_models::{PtySessionFilterModel, PtySessionInfoModel, PtySessionModel};
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

static PTY_STATE: Lazy<PtyState> = Lazy::new(|| PtyState {
	sessions: RwLock::new(Vec::new()),
});

struct PtyState {
	sessions: RwLock<Vec<Arc<PtySessionModel>>>,
}

pub async fn create_one(session: PtySessionModel) -> Result<()> {
	PTY_STATE.sessions.write().await.push(Arc::new(session));

	Ok(())
}

pub async fn get_one(id: &Uuid) -> Result<Arc<PtySessionModel>> {
	let sessions = PTY_STATE.sessions.read().await;

	let session = sessions.iter().find(|&s| s.id == *id).ok_or(Error::NotExists)?.clone();

	Ok(session)
}

pub async fn get_first(filter: PtySessionFilterModel) -> Option<Arc<PtySessionModel>> {
	let sessions = PTY_STATE.sessions.read().await;

	sessions.iter().cloned().find(|session| matches_filter(session, &filter))
}

pub async fn get_many(filter: PtySessionFilterModel) -> Result<Vec<Arc<PtySessionModel>>> {
	let sessions = PTY_STATE.sessions.read().await;

	let filtered_sessions = sessions.iter().cloned().filter(|session| matches_filter(session, &filter)).collect();

	Ok(filtered_sessions)
}

pub async fn get_many_info(filter: PtySessionFilterModel) -> Result<Vec<PtySessionInfoModel>> {
	let filtered_sessions = get_many(filter).await?;
	let mut info_models = Vec::with_capacity(filtered_sessions.len());

	for session in filtered_sessions {
		let shell_guard = session.shell.read().await;

		info_models.push(PtySessionInfoModel {
			id: session.id,
			project_id: session.meta.project_id,
			name: session.meta.name.clone(),
			task_id: session.meta.task_id,
			task_set_id: session.meta.task_set_id,
			shell_status: shell_guard.status.lock().await.clone(),
		});
	}

	Ok(info_models)
}

fn matches_filter(session: &PtySessionModel, filter: &PtySessionFilterModel) -> bool {
	let matches_id = filter.id.map_or(true, |id| session.id == id);
	let matches_project_id = filter.project_id.map_or(true, |id| session.meta.project_id == id);
	let matches_task_id = filter.task_id.map_or(true, |id| session.meta.task_id == Some(id));
	let matches_task_set_id = filter.task_set_id.map_or(true, |id| session.meta.task_set_id == Some(id));

	matches_id && matches_project_id && matches_task_id && matches_task_set_id
}

pub async fn delete_one(id: &Uuid) -> Result<()> {
	let mut sessions = PTY_STATE.sessions.write().await;
	if let Some(pos) = sessions.iter().position(|s| s.id == *id) {
		sessions.remove(pos);
	}

	Ok(())
}
