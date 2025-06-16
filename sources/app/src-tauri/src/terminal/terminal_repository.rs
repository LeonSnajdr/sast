use crate::error::Error;
use crate::prelude::*;
use crate::terminal::terminal_filters::TerminalFilter;
use crate::terminal::terminal_models::{TerminalInfoModel, TerminalOpenModel};
use crate::terminal::Terminal;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::terminal::terminal_contracts::TerminalInfoContract;

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
	let terminals = STATE.terminals.read().await;
	let terminal = terminals.iter().find(|&s| s.id == *id).ok_or(Error::NotExists)?.clone();

	Ok(terminal)
}

pub async fn get_one_open(id: &Uuid) -> Result<TerminalOpenModel> {
	let terminals = STATE.terminals.read().await;
	let terminal = terminals.iter().find(|&s| s.id == *id).ok_or(Error::NotExists)?.clone();

	let open_model = TerminalOpenModel {
		history: terminal.history.read().await.clone(),
		shell_size: terminal.meta.shell_size.read().await.clone(),
	};

	Ok(open_model)
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

pub async fn get_one_info(id: &Uuid) -> Result<TerminalInfoModel> {
	let terminal = get_one(id).await?;
	
	let info_model = to_info_model(&terminal).await;
	
	Ok(info_model)	
}

pub async fn get_many_info(filter: &TerminalFilter) -> Result<Vec<TerminalInfoModel>> {
	let terminals = get_many(filter).await?;

	let mut info_models = Vec::new();
	for terminal in terminals.iter() {
		let info_model = to_info_model(terminal).await;
		info_models.push(info_model)
	}

	Ok(info_models)
}

async fn to_info_model(terminal: &Arc<Terminal>) ->  TerminalInfoModel {
	TerminalInfoModel {
		id: terminal.id,
		name: terminal.meta.name.read().await.clone(),
		project_id: terminal.meta.project_id,
		task_id: terminal.meta.task_id,
		task_set_id: terminal.meta.task_set_id.read().await.clone(),
		shell_status: terminal.shell_status.read().await.clone(),
	}
}

async fn matches_filter(terminal: &Arc<Terminal>, filter: &TerminalFilter) -> bool {
	let terminal_meta = terminal.meta.clone();
	let terminal_shell_status = terminal.shell_status.read().await.clone();

	let matches_id = filter.id.map_or(true, |id| terminal.id == id);
	let matches_project_id = filter.project_id.map_or(true, |id| terminal_meta.project_id == id);
	let matches_task_ids = filter
		.task_ids
		.as_ref()
		.map_or(true, |task_ids| terminal_meta.task_id.map_or(false, |tid| task_ids.contains(&tid)));
	let matches_shell_status = filter
		.shell_status
		.as_ref()
		.map_or(true, |shell_status| shell_status.contains(&terminal_shell_status));

	matches_id && matches_project_id && matches_task_ids && matches_shell_status
}

pub async fn delete_one(id: &Uuid) -> Result<()> {
	let mut sessions = STATE.terminals.write().await;
	if let Some(pos) = sessions.iter().position(|s| s.id == *id) {
		sessions.remove(pos);
	}
	Ok(())
}
