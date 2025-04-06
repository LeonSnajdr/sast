use serde::{Deserialize, Serialize};

#[derive(Debug, specta::Type, Serialize, Deserialize)]
pub enum ShellKillReason {
	Manually,
	Success,
	Restart,
	Error { code: u32, message: String },
}
