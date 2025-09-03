use crate::terminal::terminal_enums::TerminalShellStatus;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;
use uuid::Uuid;

#[derive(Debug, Clone, Type, Event, Serialize, Deserialize)]
pub struct TerminalClosedEvent(pub Uuid);

#[derive(Debug, Clone, Type, Event, Serialize, Deserialize)]
pub struct TerminalCreatedEvent(pub TerminalCreatedEventData);

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalCreatedEventData {
	pub project_id: Uuid,
	pub id: Uuid,
	pub jump_into: bool,
}

#[derive(Debug, Clone, Type, Event, Serialize, Deserialize)]
pub struct TerminalUpdatedEvent(pub TerminalUpdatedEventData);

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalUpdatedEventData {
	pub project_id: Uuid,
	pub id: Uuid,
	pub jump_into: bool,
}

#[derive(Debug, Clone, Type, Event, Serialize, Deserialize)]
pub struct TerminalShellStatusChangedEvent(pub TerminalShellStatusChangedEventData);

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalShellStatusChangedEventData {
	pub project_id: Uuid,
	pub id: Uuid,
	pub status: TerminalShellStatus,
}

#[derive(Debug, Clone, Type, Event, Serialize, Deserialize)]
pub struct TerminalShellReadEvent(pub TerminalShellReadEventData);

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalShellReadEventData {
	pub id: Uuid,
	pub data: String,
}
