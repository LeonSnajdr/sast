use crate::dtos::project_dtos::{CreateProject, Project};
use crate::prelude::*;
use crate::repositories::project_repository;

pub async fn create_project(create_dto: &CreateProject) -> Result<()> {
	project_repository::create_project(create_dto).await?;

	Ok(())
}

pub async fn get_all_projects() -> Result<Vec<Project>> {
	let projects = project_repository::get_all_projects().await?;

	Ok(projects)
}
