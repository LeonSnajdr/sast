use tauri::AppHandle;
use uuid::Uuid;

use crate::prelude::*;
use crate::terminal::terminal_contracts::{TerminalFilterContract, TerminalInfoContract, TerminalResizeContract, TerminalSpawnContract};
use crate::terminal::terminal_service;

#[tauri::command]
#[specta::specta]
pub async fn terminal_spawn(app_handle: AppHandle, spawn_contract: TerminalSpawnContract) -> Result<Uuid> {
	let session_id = terminal_service::spawn(&app_handle, spawn_contract).await?;

	Ok(session_id)
}

#[tauri::command]
#[specta::specta]
pub async fn terminal_write(id: Uuid, data: String) -> Result<()> {
	terminal_service::write(id, data).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn terminal_get_read_history(id: Uuid) -> Result<String> {
	let result = terminal_service::get_read_history(id).await?;

	Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn terminal_get_many_info(filter: TerminalFilterContract) -> Result<Vec<TerminalInfoContract>> {
	let result: Vec<TerminalInfoContract> = terminal_service::get_many_info(filter).await?;

	Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn terminal_resize(id: Uuid, resize_contract: TerminalResizeContract) -> Result<()> {
	terminal_service::resize(id, resize_contract).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn terminal_kill(id: Uuid) -> Result<()> {
	terminal_service::kill(id).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn terminal_delete(app_handle: AppHandle, id: Uuid) -> Result<()> {
	terminal_service::delete(&app_handle, id).await?;

	Ok(())
}
