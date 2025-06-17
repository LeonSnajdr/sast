use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize)]
pub enum TaskSetSessionKind {
	Start,
	Restart,
}

#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize)]
pub enum TaskSetSessionStatus {
	Running,
	Failed,
	Completed,
}

#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize)]
pub enum TaskSetSessionTaskStatus {
	NotStarted,
	Running,
	Skipped,
	Failed,
	Completed,
}
