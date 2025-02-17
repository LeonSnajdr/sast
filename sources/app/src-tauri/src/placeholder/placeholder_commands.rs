use uuid::Uuid;

use crate::placeholder::placeholder_contracts::{PlaceholderContract, PlaceholderCreateContract, PlaceholderUpdateContract};
use crate::placeholder::placeholder_service;
use crate::prelude::*;

#[tauri::command]
#[specta::specta]
pub async fn placeholder_create(placeholder_create_contract: PlaceholderCreateContract) -> Result<PlaceholderContract> {
	let placeholder = placeholder_service::placeholder_create(placeholder_create_contract).await?;

	Ok(placeholder)
}

#[tauri::command]
#[specta::specta]
pub async fn placeholder_get_many(project_id: Uuid) -> Result<Vec<PlaceholderContract>> {
	let placeholders = placeholder_service::placeholder_get_many(project_id).await?;

	Ok(placeholders)
}

#[tauri::command]
#[specta::specta]
pub async fn placeholder_get_one(id: Uuid) -> Result<PlaceholderContract> {
	let placeholder = placeholder_service::placeholder_get_one(id).await?;

	Ok(placeholder)
}

#[tauri::command]
#[specta::specta]
pub async fn placeholder_update_one(placeholder_update_contract: PlaceholderUpdateContract) -> Result<()> {
	placeholder_service::placeholder_update_one(placeholder_update_contract).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn placeholder_delete_one(id: Uuid) -> Result<()> {
	placeholder_service::placeholder_delete_one(id).await?;

	Ok(())
}
