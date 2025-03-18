use crate::pty_session::pty_session_enums::{PtySessionHistoryPersistence, PtySessionShellStatus};
use portable_pty::{Child, ChildKiller, PtyPair};
use std::io::{Read, Write};
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

pub struct PtySessionModel {
	pub id: Uuid,
	pub concurrency_guard: Mutex<()>,
	pub behavior: PtySessionBehaviorModel,
	pub meta: PtySessionMetaModel,
	pub shell: RwLock<PtySessionShellModel>,
}

pub struct PtySessionBehaviorModel {
	pub history_persistence: PtySessionHistoryPersistence,
	pub force_kill: bool,
}

pub struct PtySessionMetaModel {
	pub project_id: Uuid,
	pub task_id: Option<Uuid>,
	pub task_set_id: Option<Uuid>,
	pub name: String,
	pub position: i32,
	pub history: Mutex<String>,
}

pub struct PtySessionShellModel {
	pub status: Mutex<PtySessionShellStatus>,
	pub pair: Mutex<PtyPair>,
	pub child: Mutex<Box<dyn Child + Send + Sync>>,
	pub child_killer: Mutex<Box<dyn ChildKiller + Send + Sync>>,
	pub writer: Mutex<Box<dyn Write + Send>>,
	pub reader: Mutex<Box<dyn Read + Send>>,
}

pub struct PtySessionInfoModel {
	pub id: Uuid,
	pub project_id: Uuid,
	pub task_id: Option<Uuid>,
	pub task_set_id: Option<Uuid>,
	pub name: String,
	pub position: i32,
}

pub struct PtySessionFilterModel {
	pub id: Option<Uuid>,
	pub project_id: Option<Uuid>,
	pub task_id: Option<Uuid>,
	pub task_set_id: Option<Uuid>,
}
