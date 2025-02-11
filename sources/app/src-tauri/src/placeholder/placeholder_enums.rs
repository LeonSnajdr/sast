use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::Type, specta::Type, Serialize, Deserialize)]
pub enum PlaceholderVisibility {
    Global,
    Project
}

#[derive(Debug, sqlx::Type, specta::Type, Serialize, Deserialize)]
pub enum PlaceholderKind {
    Text,
    Select
}

#[derive(Debug, sqlx::Type, specta::Type, Serialize, Deserialize)]
pub enum PlaceholderSource {
    Static
}