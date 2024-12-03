use uuid::Uuid;

use crate::{
	contracts::pty_contracts::{ResizePtyContract, SpawnPtyContract},
	prelude::*,
	services::pty_service,
};

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
pub async fn pty_resize(session_id: Uuid, resize_contract: ResizePtyContract) -> Result<()> {
	pty_service::resize(&session_id, &resize_contract).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn pty_kill(session_id: Uuid) -> Result<()> {
	pty_service::kill(&session_id).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn pty_exitstatus(session_id: Uuid) -> Result<u32> {
	let exitstatus = pty_service::exitstatus(&session_id).await?;

	Ok(exitstatus)
}
