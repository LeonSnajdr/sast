use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq)]
pub enum PtySessionShellStatus {
	Running,
	Restarting,
	Killed,
}

#[derive(Debug, PartialEq, Eq, sqlx::Type, specta::Type, Serialize, Deserialize)]
pub enum PtySessionHistoryPersistence {
	Always,
	Never,
	OnError,
	OnSuccess,
}
