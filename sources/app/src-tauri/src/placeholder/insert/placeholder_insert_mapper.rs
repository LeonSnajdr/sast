use uuid::Uuid;
use crate::placeholder::insert::placeholder_insert_contracts::PlaceholderInsertTileContract;
use crate::placeholder::insert::placeholder_insert_models::{PlaceholderInsertTileModel, PlaceholderInsertTileResultModel};

impl PlaceholderInsertTileContract {
    pub fn from(value: PlaceholderInsertTileResultModel) -> Self {
        Self {
            kind: value.kind,
            text_value: value.text_value,
            placeholder_id: value.placeholder_id,
            placeholder_name: value.placeholder_name,
        }
    }
}

impl PlaceholderInsertTileModel {
    pub fn from(id: Uuid, position: i64, task_command_id: Option<Uuid>, task_working_dir_id: Option<Uuid>, value: PlaceholderInsertTileContract) -> Self {
        Self {
            id,
            task_command_id,
            task_working_dir_id,
            placeholder_id: value.placeholder_id,
            kind: value.kind,
            position,
            text_value: value.text_value,
        }
    }
}