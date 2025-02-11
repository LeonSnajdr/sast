use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;
use crate::placeholder::placeholder_enums::{PlaceholderKind, PlaceholderSource, PlaceholderVisibility};

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceholderCreateContract {
    pub project_id: Uuid,
    pub name: String,
    pub value: String,
    pub visibility: PlaceholderVisibility,
    pub kind: PlaceholderKind,
    pub source: PlaceholderSource,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceholderUpdateContract {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub value: String,
    pub visibility: PlaceholderVisibility,
    pub kind: PlaceholderKind,
    pub source: PlaceholderSource,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceholderContract {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub value: String,
    pub visibility: PlaceholderVisibility,
    pub kind: PlaceholderKind,
    pub source: PlaceholderSource,
    pub date_created: DateTime<Utc>,
    pub date_last_updated: DateTime<Utc>,
}
