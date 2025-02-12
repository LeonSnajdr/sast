use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::Type, specta::Type, Serialize, Deserialize)]
pub enum PlaceholderInsertTileKind {
    Text,
    Placeholder
}