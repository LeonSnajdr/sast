use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnPtyContract {
	pub cols: u16,
	pub rows: u16,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResizePtyContract {
	pub cols: u16,
	pub rows: u16,
}
