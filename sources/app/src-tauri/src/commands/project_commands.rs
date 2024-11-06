use crate::contracts::project_contracts::{CreateProjectContract, ProjectContract};
use crate::prelude::*;
use crate::services::project_service;

#[tauri::command]
#[specta::specta]
pub async fn create_project(
	create_project_contract: CreateProjectContract,
) -> Result<ProjectContract> {
	let project = project_service::create_project(&create_project_contract).await?;

	Ok(project)
}

#[tauri::command]
#[specta::specta]
pub async fn get_all_projects() -> Result<Vec<ProjectContract>> {
	let projects = project_service::get_all_projects().await?;

	Ok(projects)
}

#[tauri::command]
#[specta::specta]
pub async fn get_project(id: i32) -> Result<ProjectContract> {
	let project = project_service::get_project(&id).await?;

	Ok(project)
}
