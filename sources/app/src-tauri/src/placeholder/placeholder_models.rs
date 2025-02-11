use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;
use crate::placeholder::placeholder_enums::{PlaceholderKind, PlaceholderSource, PlaceholderVisibility};

#[derive(Debug, FromRow)]
pub struct PlaceholderModel {
    pub id: Uuid,
    pub project_id: Option<Uuid>,
    pub name: String,
    pub value: String,
    pub visibility: PlaceholderVisibility,
    pub kind: PlaceholderKind,
    pub source: PlaceholderSource,
    pub date_created: DateTime<Utc>,
    pub date_last_updated: DateTime<Utc>
}

#[derive(Debug)]
pub struct PlaceholderUpdateContainer {
    pub id: Uuid,
    pub project_id: Option<Uuid>,
    pub name: String,
    pub value: String,
    pub visibility: PlaceholderVisibility,
    pub kind: PlaceholderKind,
    pub source: PlaceholderSource,
    pub date_last_updated: DateTime<Utc>
}