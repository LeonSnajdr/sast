use crate::task_set::session::task_set_session_enums::TaskSetSessionTaskStatus;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;
use uuid::Uuid;

#[derive(Debug, Clone, Type, Event, Serialize, Deserialize)]
pub struct TaskSetSessionStartedEvent(pub Uuid);

#[derive(Debug, Clone, Type, Event, Serialize, Deserialize)]
pub struct TaskSetSessionFinishedEvent(pub Uuid);

#[derive(Debug, Clone, Type, Event, Serialize, Deserialize)]
pub struct TaskSetSessionTaskStatusChangedEvent(pub TaskSetSessionTaskStatusChangedEventData);

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSetSessionTaskStatusChangedEventData {
	pub task_set_session_id: Uuid,
	pub task_id: Uuid,
	pub status: TaskSetSessionTaskStatus,
}
