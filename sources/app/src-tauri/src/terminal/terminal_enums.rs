use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, specta::Type, Serialize, Deserialize)]
pub enum TerminalShellStatus {
	Creating,
	Running,
	Restarting,
	Killing,
	Killed,
	Failed,
}

#[derive(Debug, PartialEq, Eq, Clone, sqlx::Type, specta::Type, Serialize, Deserialize)]
pub enum TerminalHistoryPersistence {
	Always,
	Never,
	OnError,
	OnSuccess,
}
