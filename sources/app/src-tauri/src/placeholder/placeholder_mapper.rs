use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::placeholder::placeholder_contracts::{PlaceholderContract, PlaceholderCreateContract, PlaceholderUpdateContract};
use crate::placeholder::placeholder_models::{PlaceholderModel, PlaceholderUpdateContainer};

impl From<PlaceholderModel> for PlaceholderContract {
    fn from(value: PlaceholderModel) -> Self {
        Self {
            id: value.id,
            project_id: value.project_id,
            name: value.name,
            value: value.value,
            visibility: value.visibility,
            kind: value.kind,
            source: value.source,
            date_created: value.date_created,
            date_last_updated: value.date_last_updated
        }
    }
}

impl PlaceholderModel {
    pub fn from_create_contract(id: Uuid, date_created: DateTime<Utc>, date_last_updated: DateTime<Utc>, value: PlaceholderCreateContract) -> Self {
        Self {
            id,
            project_id: value.project_id,
            name: value.name,
            value: value.value,
            visibility: value.visibility,
            kind: value.kind,
            source: value.source,
            date_created,
            date_last_updated
        }
    }
}

impl PlaceholderUpdateContainer {
    pub fn from(date_last_updated: DateTime<Utc>, value: PlaceholderUpdateContract) -> Self {
        Self {
            id: value.id,
            project_id: value.project_id,
            name: value.name,
            value: value.value,
            visibility: value.visibility,
            kind: value.kind,
            source: value.source,
            date_last_updated
        }
    }
}