use crate::task_set::session::task_set_session_enums::{TaskSetSessionKind, TaskSetSessionStatus, TaskSetSessionTaskStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSetSessionContract {
	pub id: Uuid,
	pub project_id: Uuid,
	pub task_set_id: Uuid,
	pub kind: TaskSetSessionKind,
	pub date_started: DateTime<Utc>,
	pub date_finished: Option<DateTime<Utc>>,
	pub status: TaskSetSessionStatus,
	pub tasks: Vec<TaskSetSessionTaskContract>,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSetSessionTaskContract {
	pub task_id: Uuid,
	pub task_name: String,
	pub date_started: Option<DateTime<Utc>>,
	pub date_finished: Option<DateTime<Utc>>,
	pub status: TaskSetSessionTaskStatus,
}
