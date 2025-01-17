use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct PlaceholderModel {
    pub id: Uuid,
    pub project_id: Option<Uuid>,
    pub name: String,
    pub value: String,
    pub date_created: DateTime<Utc>,
    pub date_last_updated: DateTime<Utc>
}