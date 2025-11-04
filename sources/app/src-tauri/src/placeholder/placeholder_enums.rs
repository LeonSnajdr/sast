use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, sqlx::Type, specta::Type, Serialize, Deserialize)]
pub enum PlaceholderVisibility {
	Global,
	Project,
}

#[derive(Debug, Clone, sqlx::Type, specta::Type, Serialize, Deserialize)]
pub enum PlaceholderKind {
	Text,
	Select,
}

#[derive(Debug, Clone, sqlx::Type, specta::Type, Serialize, Deserialize)]
pub enum PlaceholderSource {
	Static,
}
