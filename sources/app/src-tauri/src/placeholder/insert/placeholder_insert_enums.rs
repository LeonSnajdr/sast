use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, sqlx::Type, specta::Type, Serialize, Deserialize)]
pub enum PlaceholderInsertTileKind {
	Text,
	Placeholder,
}
