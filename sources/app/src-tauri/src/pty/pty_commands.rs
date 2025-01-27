use uuid::Uuid;

use crate::prelude::*;
use crate::pty::pty_contracts::{PtyInfoContract, PtyResizeContract, PtySpawnContract};
use crate::pty::pty_service;

#[tauri::command]
#[specta::specta]
pub async fn pty_spawn(spawn_contract: PtySpawnContract) -> Result<Uuid> {
	let session_id = pty_service::pty_spawn(&spawn_contract).await?;

	Ok(session_id)
}

#[tauri::command]
#[specta::specta]
pub async fn pty_write(session_id: Uuid, data: String) -> Result<()> {
	pty_service::pty_write(&session_id, &data).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn pty_read(session_id: Uuid) -> Result<String> {
	let result = pty_service::pty_read(&session_id).await?;

	Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn pty_get_read_history(session_id: Uuid) -> Result<Vec<String>> {
	let result = pty_service::get_read_history(&session_id).await?;

	Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn pty_get_sessions(project_id: Uuid) -> Result<Vec<PtyInfoContract>> {
	let result: Vec<PtyInfoContract> = pty_service::pty_get_sessions(&project_id).await?;

	Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn pty_resize(session_id: Uuid, resize_contract: PtyResizeContract) -> Result<()> {
	pty_service::pty_resize(&session_id, &resize_contract).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn pty_kill(session_id: Uuid) -> Result<()> {
	pty_service::pty_kill(&session_id).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn pty_exitstatus(session_id: Uuid) -> Result<u32> {
	let exitstatus = pty_service::pty_exitstatus(&session_id).await?;

	Ok(exitstatus)
}
