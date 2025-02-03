use uuid::Uuid;
use tauri::AppHandle;

use crate::prelude::*;
use crate::pty_session::pty_session_contracts::{PtySessionInfoContract, PtySessionResizeContract, PtySessionSpawnContract};
use crate::pty_session::pty_session_service;

#[tauri::command]
#[specta::specta]
pub async fn pty_session_spawn(app_handle: AppHandle, spawn_contract: PtySessionSpawnContract) -> Result<Uuid> {
	let session_id = pty_session_service::pty_session_spawn(app_handle, spawn_contract).await?;

	Ok(session_id)
}

#[tauri::command]
#[specta::specta]
pub async fn pty_session_write(session_id: Uuid, data: String) -> Result<()> {
	pty_session_service::pty_session_write(&session_id, &data).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn pty_session_get_read_history(session_id: Uuid) -> Result<String> {
	let result = pty_session_service::pty_session_get_read_history(&session_id).await?;

	Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn pty_session_info_get_all(project_id: Uuid) -> Result<Vec<PtySessionInfoContract>> {
	let result: Vec<PtySessionInfoContract> = pty_session_service::pty_session_info_get_all(&project_id).await?;

	Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn pty_session_resize(session_id: Uuid, resize_contract: PtySessionResizeContract) -> Result<()> {
	pty_session_service::pty_session_resize(&session_id, &resize_contract).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn pty_session_kill(session_id: Uuid) -> Result<()> {
	pty_session_service::pty_session_kill(&session_id).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn pty_session_get_exitstatus(session_id: Uuid) -> Result<u32> {
	let exitstatus = pty_session_service::pty_session_get_exitstatus(&session_id).await?;

	Ok(exitstatus)
}
