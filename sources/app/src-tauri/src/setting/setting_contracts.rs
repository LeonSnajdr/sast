use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingInitializeContract {
	pub presentation_language: String,
	pub presentation_theme: String,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingUpdateContract {
	pub id: Uuid,
	pub presentation_language: String,
	pub presentation_theme: String,
	pub behavior_open_welcome: bool,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingContract {
	pub id: Uuid,
	pub meta_date_updated: DateTime<Utc>,
	pub presentation_language: String,
	pub presentation_theme: String,
	pub behavior_open_welcome: bool,
}
