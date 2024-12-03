use uuid::Uuid;

use crate::{contracts::pty_contracts::SpawnPtyContract, prelude::*, services::pty_service};

#[tauri::command]
#[specta::specta]
pub async fn pty_spawn(spawn_contract: SpawnPtyContract) -> Result<Uuid> {
	let session_id = pty_service::spawn(&spawn_contract).await?;

	Ok(session_id)
}

#[tauri::command]
#[specta::specta]
pub async fn pty_write(session_id: Uuid, data: String) -> Result<()> {
	pty_service::write(&session_id, &data).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn pty_read(session_id: Uuid) -> Result<String> {
	let result = pty_service::read(&session_id).await?;

	Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn pty_resize() -> Result<()> {
	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn pty_kill() -> Result<()> {
	Ok(())
}
