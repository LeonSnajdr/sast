use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PtySpawnContract {
	pub project_id: Uuid,
	pub name: String,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PtyInfoContract {
	pub session_id: Uuid,
	pub project_id: Uuid,
	pub name: String,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PtyResizeContract {
	pub cols: u16,
	pub rows: u16,
}
