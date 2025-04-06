use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, specta::Type, Serialize, Deserialize)]
pub enum TerminalShellStatus {
	None,
	Running,
	Restarting,
	Killed,
	Crashed { code: u32, message: String },
}

#[derive(Debug, PartialEq, Eq, Clone, sqlx::Type, specta::Type, Serialize, Deserialize)]
pub enum TerminalHistoryPersistence {
	Always,
	Never,
	OnError,
	OnSuccess,
}
