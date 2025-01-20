use uuid::Uuid;

use crate::prelude::*;
use crate::placeholder::placeholder_service;
use crate::placeholder::placeholder_contracts::{PlaceholderCreateContract, PlaceholderContract};

#[tauri::command]
#[specta::specta]
pub async fn placeholder_create(placeholder_create_contract: PlaceholderCreateContract) -> Result<PlaceholderContract> {
    let placeholder = placeholder_service::placeholder_create(&placeholder_create_contract).await?;

    Ok(placeholder)
}

#[tauri::command]
#[specta::specta]
pub async fn placeholder_get_all_global() -> Result<Vec<PlaceholderContract>> {
    let placeholders = placeholder_service::placeholder_get_all_global().await?;

    Ok(placeholders)
}

#[tauri::command]
#[specta::specta]
pub async fn placeholders_get_all_project(project_id: Option<Uuid>) -> Result<Vec<PlaceholderContract>> {
    let placeholders = placeholder_service::placeholder_get_all_project(&project_id).await?;

    Ok(placeholders)
}

#[tauri::command]
#[specta::specta]
pub async fn placeholder_get_one(id: Uuid) -> Result<PlaceholderContract> {
    let placeholder = placeholder_service::placeholder_get_one(&id).await?;

    Ok(placeholder)
}
