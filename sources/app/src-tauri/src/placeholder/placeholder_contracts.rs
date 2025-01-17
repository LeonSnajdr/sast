use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePlaceholderContract {
    pub project_id: Option<Uuid>,
    pub name: String,
    pub value: String
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceholderContract {
    pub id: Uuid,
    pub project_id: Option<Uuid>,
    pub name: String,
    pub value: String,
    pub date_created: DateTime<Utc>,
    pub date_last_updated: DateTime<Utc>,
}
