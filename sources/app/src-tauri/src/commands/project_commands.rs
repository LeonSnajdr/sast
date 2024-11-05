use crate::dtos::project_dtos::{CreateProject, Project};
use crate::prelude::*;
use crate::services::project_service;

#[tauri::command]
#[specta::specta]
pub async fn create_project(create_dto: CreateProject) -> Result<()> {
	project_service::create_project(&create_dto).await?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_all_projects() -> Result<Vec<Project>> {
	let projects = project_service::get_all_projects().await?;

	Ok(projects)
}
