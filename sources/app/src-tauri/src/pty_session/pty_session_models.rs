use portable_pty::{Child, ChildKiller, PtyPair};
use std::io::{Read, Write};
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct PtySessionModel {
	pub id: Uuid,
	pub project_id: Uuid,
	pub task_id: Option<Uuid>,
	pub task_set_id: Option<Uuid>,
	pub name: String,
	pub pair: Mutex<PtyPair>,
	pub child: Mutex<Box<dyn Child + Send + Sync>>,
	pub child_killer: Mutex<Box<dyn ChildKiller + Send + Sync>>,
	pub writer: Mutex<Box<dyn Write + Send>>,
	pub reader: Mutex<Box<dyn Read + Send>>,
	pub read_history: Mutex<String>,
}

pub struct PtySessionInfoModel {
	pub id: Uuid,
	pub project_id: Uuid,
	pub task_id: Option<Uuid>,
	pub task_set_id: Option<Uuid>,
	pub name: String,
}

pub struct PtySessionFilterModel {
	pub project_id: Option<Uuid>,
	pub task_id: Option<Uuid>,
	pub task_set_id: Option<Uuid>,
}
