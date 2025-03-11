use tauri::AppHandle;
use uuid::Uuid;

use crate::prelude::*;
use crate::pty_session::pty_session_contracts::{PtySessionFilterContract, PtySessionInfoContract, PtySessionResizeContract, PtySessionSpawnContract};
use crate::pty_session::pty_session_service;

#[tauri::command]
#[specta::specta]
pub async fn pty_session_spawn(app_handle: AppHandle, spawn_contract: PtySessionSpawnContract) -> Result<Uuid> {
	let session_id = pty_session_service::spawn(&app_handle, spawn_contract).await?;

	Ok(session_id)
}

#[tauri::command]
#[specta::specta]
pub async fn pty_session_write(id: Uuid, data: String) -> Result<()> {
	pty_session_service::write(&id, &data).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn pty_session_get_read_history(id: Uuid) -> Result<String> {
	let result = pty_session_service::get_read_history(&id).await?;

	Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn pty_session_get_many_info(filter: PtySessionFilterContract) -> Result<Vec<PtySessionInfoContract>> {
	let result: Vec<PtySessionInfoContract> = pty_session_service::get_many_info(filter).await?;

	Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn pty_session_resize(id: Uuid, resize_contract: PtySessionResizeContract) -> Result<()> {
	pty_session_service::resize(&id, &resize_contract).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn pty_session_kill(id: Uuid) -> Result<()> {
	pty_session_service::kill(&id).await?;

	Ok(())
}
