use serde::{Serialize, Deserialize};
use specta::Type;
use tauri_specta::Event;
use uuid::Uuid;

#[derive(Debug, Clone, Type, Event, Serialize, Deserialize)]
pub struct PtySessionsUpdatedEvent();

#[derive(Debug, Clone, Type, Event, Serialize, Deserialize)]
pub struct PtySessionReadEvent(pub PtySessionReadEventData);

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PtySessionReadEventData {
    pub session_id: Uuid,
    pub data: String,
}
