use crate::error::Error;
use crate::prelude::*;
use crate::terminal::terminal_filters::TerminalFilter;
use crate::terminal::terminal_models::TerminalInfoModel;
use crate::terminal::Terminal;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

static STATE: Lazy<PtyState> = Lazy::new(|| PtyState {
	terminals: RwLock::new(Vec::new()),
});

struct PtyState {
	terminals: RwLock<Vec<Arc<Terminal>>>,
}

pub async fn create_one(terminal: Terminal) -> Result<Arc<Terminal>> {
	let terminal_arc = Arc::new(terminal);
	let terminal_return = Arc::clone(&terminal_arc);

	STATE.terminals.write().await.push(terminal_arc);

	Ok(terminal_return)
}

pub async fn get_one(id: &Uuid) -> Result<Arc<Terminal>> {
	let sessions = STATE.terminals.read().await;
	let session = sessions.iter().find(|&s| s.id == *id).ok_or(Error::NotExists)?.clone();

	Ok(session)
}

pub async fn get_first(filter: &TerminalFilter) -> Option<Arc<Terminal>> {
	let terminals = STATE.terminals.read().await;
	for terminal in terminals.iter().cloned() {
		if matches_filter(&terminal, filter).await {
			return Some(terminal);
		}
	}

	None
}

pub async fn get_is_existing(filter: &TerminalFilter) -> bool {
	get_first(filter).await.is_some()
}

pub async fn get_many(filter: &TerminalFilter) -> Result<Vec<Arc<Terminal>>> {
	let terminals = STATE.terminals.read().await;

	let mut filtered_terminals = Vec::new();
	for terminal in terminals.iter().cloned() {
		if matches_filter(&terminal, filter).await {
			filtered_terminals.push(terminal);
		}
	}

	Ok(filtered_terminals)
}

pub async fn get_many_info(filter: &TerminalFilter) -> Result<Vec<TerminalInfoModel>> {
	let terminals = get_many(filter).await?;

	let mut info_models = Vec::new();
	for terminal in terminals.iter() {
		let meta = terminal.meta.read().await;

		info_models.push(TerminalInfoModel {
			id: terminal.id,
			name: meta.name.clone(),
			project_id: meta.project_id,
			task_id: meta.task_id,
			shell_status: terminal.shell_status.read().await.clone(),
		})
	}

	Ok(info_models)
}

async fn matches_filter(terminal: &Arc<Terminal>, filter: &TerminalFilter) -> bool {
	let meta_guard = terminal.meta.read().await;

	let matches_id = filter.id.map_or(true, |id| terminal.id == id);
	let matches_project_id = filter.project_id.map_or(true, |id| meta_guard.project_id == id);
	let matches_task_ids = filter
		.task_ids
		.as_ref()
		.map_or(true, |task_ids| meta_guard.task_id.map_or(false, |tid| task_ids.contains(&tid)));

	matches_id && matches_project_id && matches_task_ids
}

pub async fn delete_one(id: &Uuid) -> Result<()> {
	let mut sessions = STATE.terminals.write().await;
	if let Some(pos) = sessions.iter().position(|s| s.id == *id) {
		sessions.remove(pos);
	}
	Ok(())
}
