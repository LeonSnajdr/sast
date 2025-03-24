use crate::terminal::terminal_enums::{TerminalHistoryPersistence, TerminalShellStatus};
use portable_pty::{Child, ChildKiller, PtyPair};
use std::io::{Read, Write};
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

pub struct TerminalModel {
	pub id: Uuid,
	pub concurrency_guard: Mutex<()>,
	pub history: Mutex<String>,
	pub behavior: RwLock<TerminalBehaviorModel>,
	pub meta: RwLock<TerminalMetaModel>,
	pub shell: RwLock<TerminalShellModel>,
}

pub struct TerminalBehaviorModel {
	pub history_persistence: TerminalHistoryPersistence,
	pub force_kill: bool,
}

pub struct TerminalMetaModel {
	pub project_id: Uuid,
	pub task_id: Option<Uuid>,
	pub task_set_id: Option<Uuid>,
	pub name: String,
}

pub struct TerminalShellModel {
	pub status: Mutex<TerminalShellStatus>,
	pub pair: Mutex<PtyPair>,
	pub child: Mutex<Box<dyn Child + Send + Sync>>,
	pub child_killer: Mutex<Box<dyn ChildKiller + Send + Sync>>,
	pub writer: Mutex<Box<dyn Write + Send>>,
	pub reader: Mutex<Box<dyn Read + Send>>,
}

pub struct TerminalInfoModel {
	pub id: Uuid,
	pub project_id: Uuid,
	pub task_id: Option<Uuid>,
	pub task_set_id: Option<Uuid>,
	pub name: String,
	pub shell_status: TerminalShellStatus,
}

pub struct TerminalFilterModel {
	pub id: Option<Uuid>,
	pub project_id: Option<Uuid>,
	pub task_id: Option<Uuid>,
	pub task_set_id: Option<Uuid>,
}
