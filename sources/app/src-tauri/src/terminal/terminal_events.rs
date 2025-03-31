use crate::terminal::terminal_enums::TerminalShellStatus;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;
use uuid::Uuid;

#[derive(Debug, Clone, Type, Event, Serialize, Deserialize)]
pub struct TerminalDeletedEvent(pub Uuid);

#[derive(Debug, Clone, Type, Event, Serialize, Deserialize)]
pub struct TerminalCreatedEvent(pub Uuid);

#[derive(Debug, Clone, Type, Event, Serialize, Deserialize)]
pub struct TerminalStatusChangedEvent(pub TerminalStatusChangedEventData);

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalStatusChangedEventData {
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
