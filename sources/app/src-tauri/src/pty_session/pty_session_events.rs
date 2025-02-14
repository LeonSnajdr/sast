use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;
use uuid::Uuid;

#[derive(Debug, Clone, Type, Event, Serialize, Deserialize)]
pub struct PtySessionSpawnedEvent(pub Uuid);

#[derive(Debug, Clone, Type, Event, Serialize, Deserialize)]
pub struct PtySessionKilledEvent(pub Uuid);

#[derive(Debug, Clone, Type, Event, Serialize, Deserialize)]
pub struct PtySessionReadEvent(pub PtySessionReadEventData);

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PtySessionReadEventData {
	pub session_id: Uuid,
	pub data: String,
}
