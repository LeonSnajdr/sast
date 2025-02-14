use sqlx::FromRow;
use uuid::Uuid;
use crate::placeholder::insert::placeholder_insert_enums::PlaceholderInsertTileKind;
use crate::placeholder::placeholder_enums::PlaceholderVisibility;

#[derive(Debug)]
pub struct PlaceholderInsertTileModel {
    pub id: Uuid,
    pub task_command_id: Option<Uuid>,
    pub task_working_dir_id: Option<Uuid>,
    pub placeholder_id: Option<Uuid>,
    pub kind: PlaceholderInsertTileKind,
    pub position: i64,
    pub text_value: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct PlaceholderInsertTileResultModel {
    pub kind: PlaceholderInsertTileKind,
    pub position: i64,
    pub text_value: Option<String>,
    pub placeholder_id: Option<Uuid>,
    pub placeholder_name: Option<String>,
    pub placeholder_visibility: Option<PlaceholderVisibility>,
}

#[derive(Debug)]
pub struct PlaceholderInsertTileFilterModel {
    pub task_command_id: Option<Uuid>,
    pub task_working_dir_id: Option<Uuid>
}