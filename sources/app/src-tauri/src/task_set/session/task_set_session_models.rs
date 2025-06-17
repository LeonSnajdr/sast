use crate::task_set::session::task_set_session_enums::{TaskSetSessionKind, TaskSetSessionStatus, TaskSetSessionTaskStatus};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;


// TODO check if the arcs around rw locks are needed
#[derive(Debug)]
pub struct TaskSetSessionModel {
	pub id: Uuid,
	pub project_id: Uuid,
	pub task_set_id: Uuid,
	pub kind: TaskSetSessionKind,
	pub date_started: DateTime<Utc>,
	pub date_finished: Arc<RwLock<Option<DateTime<Utc>>>>,
	pub status: Arc<RwLock<TaskSetSessionStatus>>,
	pub tasks: Arc<RwLock<Vec<TaskSetSessionTaskModel>>>,
}

#[derive(Debug)]
pub struct TaskSetSessionInfoModel {
	pub id: Uuid,
	pub project_id: Uuid,
	pub task_set_id: Uuid,
	pub kind: TaskSetSessionKind,
	pub date_started: DateTime<Utc>,
	pub date_finished: Option<DateTime<Utc>>,
	pub status: TaskSetSessionStatus,
	pub tasks: Vec<TaskSetSessionTaskInfoModel>,
}

#[derive(Debug)]
pub struct TaskSetSessionTaskModel {
	pub task_id: Uuid,
	pub task_name: String,
	pub date_started: Arc<RwLock<Option<DateTime<Utc>>>>,
	pub date_finished: Arc<RwLock<Option<DateTime<Utc>>>>,
	pub status: Arc<RwLock<TaskSetSessionTaskStatus>>,
}

#[derive(Debug)]
pub struct TaskSetSessionTaskInfoModel {
	pub task_id: Uuid,
	pub task_name: String,
	pub date_started: Option<DateTime<Utc>>,
	pub date_finished: Option<DateTime<Utc>>,
	pub status: TaskSetSessionTaskStatus,
}
