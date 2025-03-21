use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, specta::Type, Serialize, Deserialize)]
pub enum PtySessionShellStatus {
	Creating,
	Running,
	Restarting,
	Killing,
	Killed,
}

#[derive(Debug, PartialEq, Eq, Clone, sqlx::Type, specta::Type, Serialize, Deserialize)]
pub enum PtySessionHistoryPersistence {
	Always,
	Never,
	OnError,
	OnSuccess,
}
