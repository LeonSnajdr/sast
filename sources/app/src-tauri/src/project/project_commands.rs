use uuid::Uuid;

use crate::prelude::*;
use crate::project::project_contracts::{ProjectContract, ProjectCreateContract, ProjectUpdateContract};
use crate::project::project_service;

#[tauri::command]
#[specta::specta]
pub async fn project_create(project_create_contract: ProjectCreateContract) -> Result<ProjectContract> {
	let project = project_service::create(&project_create_contract).await?;

	Ok(project)
}

#[tauri::command]
#[specta::specta]
pub async fn project_get_all() -> Result<Vec<ProjectContract>> {
	let projects = project_service::get_all().await?;

	Ok(projects)
}

#[tauri::command]
#[specta::specta]
pub async fn project_open(id: Uuid) -> Result<ProjectContract> {
	let project = project_service::open(&id).await?;

	Ok(project)
}

#[tauri::command]
#[specta::specta]
pub async fn project_get_one(id: Uuid) -> Result<ProjectContract> {
	let project = project_service::get_one(&id).await?;

	Ok(project)
}

#[tauri::command]
#[specta::specta]
pub async fn project_get_last_opened() -> Result<Option<ProjectContract>> {
	let project = project_service::get_last_opened().await?;

	Ok(project)
}

#[tauri::command]
#[specta::specta]
pub async fn project_update_one(update_contract: ProjectUpdateContract) -> Result<ProjectContract> {
	let project = project_service::update_one(update_contract).await?;

	Ok(project)
}

#[tauri::command]
#[specta::specta]
pub async fn project_delete_one(id: Uuid) -> Result<()> {
	project_service::delete_one(id).await?;

	Ok(())
}
