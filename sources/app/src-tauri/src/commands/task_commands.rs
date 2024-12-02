use uuid::Uuid;

use crate::{contracts::task_contracts::SpawnTaskContract, prelude::*, services::task_service};

#[tauri::command]
#[specta::specta]
pub async fn spawn_task(spawn_contract: SpawnTaskContract) -> Result<Uuid> {
	let session_id = task_service::spawn(&spawn_contract).await?;

	Ok(session_id)
}

#[tauri::command]
#[specta::specta]
pub async fn write_to_task(session_id: Uuid, data: String) -> Result<()> {
	task_service::write(&session_id, &data).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn read_from_task(session_id: Uuid) -> Result<String> {
	let result = task_service::read(&session_id).await?;

	Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn resize_task() -> Result<()> {
	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn kill_task() -> Result<()> {
	Ok(())
}
