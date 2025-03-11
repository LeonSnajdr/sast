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

	drop(sessions);

	Ok(session)
}

pub async fn get_many_info(filter: PtySessionFilterModel) -> Result<Vec<PtySessionInfoModel>> {
	let sessions = PTY_STATE.sessions.read().await;

	let filtered_sessions = sessions
		.iter()
		.map(|session| PtySessionInfoModel {
			id: session.id,
			project_id: session.project_id,
			name: session.name.clone(),
			task_id: session.task_id,
			task_set_id: session.task_set_id,
		})
		.filter(|session| matches_filter(session, &filter))
		.collect();

	Ok(filtered_sessions)
}

pub async fn delete_one(id: Uuid) -> Result<()> {
	let mut sessions = PTY_STATE.sessions.write().await;
	if let Some(pos) = sessions.iter().position(|s| s.id == id) {
		sessions.remove(pos);
	}

	Ok(())
}

fn matches_filter(session: &PtySessionInfoModel, filter: &PtySessionFilterModel) -> bool {
	let matches_project_id = filter.project_id.map_or(true, |id| session.project_id == id);
	let matches_task_id = filter.task_id.map_or(true, |id| session.task_id == Some(id));
	let matches_task_set_id = filter.task_set_id.map_or(true, |id| session.task_set_id == Some(id));

	matches_project_id && matches_task_id && matches_task_set_id
}
