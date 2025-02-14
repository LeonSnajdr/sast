use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

use crate::placeholder::insert::placeholder_insert_enums::PlaceholderInsertTileKind;
use crate::placeholder::placeholder_enums::PlaceholderVisibility;

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceholderInsertTileContract {
    pub kind: PlaceholderInsertTileKind,
    pub text_value: Option<String>,
    pub placeholder_id: Option<Uuid>,
    pub placeholder_name: Option<String>,
    pub placeholder_visibility: Option<PlaceholderVisibility>
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceholderInsertTileFilterContract {
    pub task_command_id: Option<Uuid>,
    pub task_working_dir_id: Option<Uuid>
}

impl Default for PlaceholderInsertTileFilterContract {
    fn default() -> Self {
        Self {
            task_command_id: None,
            task_working_dir_id: None,
        }
    }
}