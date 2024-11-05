use crate::contracts::project_contracts::{CreateProjectContract, ProjectContract};
use crate::prelude::*;
use crate::services::project_service;

#[tauri::command]
#[specta::specta]
pub async fn create_project(create_dto: CreateProjectContract) -> Result<()> {
	project_service::create_project(&create_dto).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_all_projects() -> Result<Vec<ProjectContract>> {
	let projects = project_service::get_all_projects().await?;

	Ok(projects)
}
