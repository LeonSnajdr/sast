use uuid::Uuid;

use crate::prelude::*;
use crate::project::project_contracts::{ProjectContract, ProjectCreateContract};
use crate::project::project_service;

#[tauri::command]
#[specta::specta]
pub async fn project_create(project_create_contract: ProjectCreateContract) -> Result<ProjectContract> {
	let project = project_service::project_create(&project_create_contract).await?;

	Ok(project)
}

#[tauri::command]
#[specta::specta]
pub async fn project_get_all() -> Result<Vec<ProjectContract>> {
	let projects = project_service::project_get_all().await?;

	Ok(projects)
}

#[tauri::command]
#[specta::specta]
pub async fn project_open(id: Uuid) -> Result<ProjectContract> {
	let project = project_service::open_project(&id).await?;

	Ok(project)
}

#[tauri::command]
#[specta::specta]
pub async fn project_get_id_last_opened() -> Result<Option<Uuid>> {
	let project = project_service::project_get_id_last_opened().await?;

	Ok(project)
}
