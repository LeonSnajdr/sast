use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeSettingContract {
	pub presentation_language: String,
	pub presentation_theme: String,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingContract {
	pub presentation_language: Option<String>,
	pub presentation_theme: Option<String>,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingContract {
	pub id: Uuid,
	pub meta_date_updated: DateTime<Utc>,
	pub presentation_language: String,
	pub presentation_theme: String,
}
