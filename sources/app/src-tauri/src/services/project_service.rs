use crate::contracts::project_contracts::{CreateProjectContract, ProjectContract};
use crate::prelude::*;
use crate::repositories::project_repository;

pub async fn create_project(create_dto: &CreateProjectContract) -> Result<()> {
	project_repository::create_project(&create_dto.name).await?;

	Ok(())
}

pub async fn get_all_projects() -> Result<Vec<ProjectContract>> {
	let project_models = project_repository::get_all_projects().await?;

	let project_contracts: Vec<ProjectContract> = project_models
		.into_iter()
		.map(ProjectContract::from)
		.collect();

	Ok(project_contracts)
}
