use uuid::Uuid;

use crate::prelude::*;
use crate::placeholder::placeholder_service;
use crate::placeholder::placeholder_contracts::{CreatePlaceholderContract, PlaceholderContract};

#[tauri::command]
#[specta::specta]
pub async fn create_placeholder(create_placeholder_contract: CreatePlaceholderContract) -> Result<PlaceholderContract> {
    let placeholder = placeholder_service::create_placeholder(&create_placeholder_contract).await?;

    Ok(placeholder)
}

#[tauri::command]
#[specta::specta]
pub async fn get_global_placeholders() -> Result<Vec<PlaceholderContract>> {
    let placeholders = placeholder_service::get_global_placeholders().await?;

    Ok(placeholders)
}

#[tauri::command]
#[specta::specta]
pub async fn get_project_placeholders(project_id: Option<Uuid>) -> Result<Vec<PlaceholderContract>> {
    let placeholders = placeholder_service::get_project_placeholders(&project_id).await?;

    Ok(placeholders)
}

#[tauri::command]
#[specta::specta]
pub async fn get_placeholder(id: Uuid) -> Result<PlaceholderContract> {
    let placeholder = placeholder_service::get_placeholder(&id).await?;

    Ok(placeholder)
}
