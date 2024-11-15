use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct SettingModel {
	pub id: Uuid,
	pub meta_date_updated: DateTime<Utc>,
	pub presentation_language: String,
	pub presentation_theme: String,
}
