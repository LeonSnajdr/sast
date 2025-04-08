use tauri::AppHandle;
use uuid::Uuid;

use crate::prelude::*;
use crate::terminal::shell::shell_contracts::{ShellResizeContract, ShellSpawnContract};
use crate::terminal::terminal_contracts::{TerminalCreateContract, TerminalInfoContract};
use crate::terminal::terminal_filters::TerminalFilter;
use crate::terminal::terminal_service;

#[tauri::command]
#[specta::specta]
pub async fn terminal_create(app_handle: AppHandle, create_contract: TerminalCreateContract, spawn_contract: Option<ShellSpawnContract>) -> Result<Uuid> {
	let id = terminal_service::create(app_handle, create_contract, spawn_contract).await?;

	Ok(id)
}

#[tauri::command]
#[specta::specta]
pub async fn terminal_get_many_info(filter: TerminalFilter) -> Result<Vec<TerminalInfoContract>> {
	let result = terminal_service::get_many_info(&filter).await?;

	Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn terminal_get_history(id: Uuid) -> Result<String> {
	let result = terminal_service::get_history(id).await?;

	Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn terminal_close(id: Uuid) -> Result<()> {
	terminal_service::close(id).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn terminal_shell_write(id: Uuid, data: String) -> Result<()> {
	terminal_service::shell_write(id, data).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn terminal_shell_resize(id: Uuid, resize_contract: ShellResizeContract) -> Result<()> {
	terminal_service::shell_resize(id, resize_contract).await?;

	Ok(())
}
