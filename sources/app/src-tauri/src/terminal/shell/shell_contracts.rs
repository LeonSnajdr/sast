use crate::terminal::terminal_enums::TerminalHistoryPersistence;
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellResizeContract {
	pub rows: u16,
	pub cols: u16,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellSpawnContract {
	pub working_dir: Option<String>,
	pub command: Option<String>,
	pub no_exit: bool,
	pub force_kill: bool,
}
