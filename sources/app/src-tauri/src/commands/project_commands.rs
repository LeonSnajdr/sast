use crate::dtos::project_dtos::ProjectDto;
use crate::prelude::*;
use crate::services::project_service;

#[tauri::command]
pub async fn create_project() -> Result<()> {
	project_service::create_project().await?;

	Ok(())
}

#[tauri::command]
pub async fn get_all_projects() -> Result<Vec<ProjectDto>> {
	let projects = project_service::get_all_projects().await?;

	Ok(projects)
}
