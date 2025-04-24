use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellSizeContract {
	pub rows: u16,
	pub cols: u16,
}

impl Default for ShellSizeContract {
	fn default() -> Self {
		ShellSizeContract { rows: 24, cols: 80 }
	}
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellSpawnContract {
	pub working_dir: Option<String>,
	pub command: Option<String>,
	pub no_exit: bool,
	pub force_kill: bool,
}
