use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSetSessionFilter {
	pub id: Option<Uuid>,
	pub project_id: Option<Uuid>,
	pub task_set_id: Option<Uuid>,
}

impl Default for TaskSetSessionFilter {
	fn default() -> Self {
		Self { 
			id: None,
			project_id: None,
			task_set_id: None 
		}
	}
}
