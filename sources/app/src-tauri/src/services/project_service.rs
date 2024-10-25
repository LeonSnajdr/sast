use crate::dtos::project_dtos::ProjectDto;
use crate::prelude::*;
use crate::repositories::project_repository;

pub async fn create_project() -> Result<()> {
	project_repository::create_project().await?;

	Ok(())
}

pub async fn get_all_projects() -> Result<Vec<ProjectDto>> {
	let projects = project_repository::get_all_projects().await?;

	Ok(projects)
}
