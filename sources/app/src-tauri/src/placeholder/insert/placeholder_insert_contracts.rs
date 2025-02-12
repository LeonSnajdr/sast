use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

use crate::placeholder::insert::placeholder_insert_enums::PlaceholderInsertTileKind;

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceholderInsertTileContract {
    pub kind: PlaceholderInsertTileKind,
    pub text_value: Option<String>,
    pub placeholder_id: Option<Uuid>,
    pub placeholder_name: Option<String>
}