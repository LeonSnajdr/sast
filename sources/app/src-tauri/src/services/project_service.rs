use crate::contracts::project_contracts::{CreateProjectContract, ProjectContract};
use crate::prelude::*;
use crate::repositories::project_repository;

pub async fn create_project(
	create_project_contract: &CreateProjectContract,
) -> Result<ProjectContract> {
	let project_model = project_repository::create_project(&create_project_contract.name).await?;

	let project_contract = ProjectContract::from(project_model);

	Ok(project_contract)
}

pub async fn get_all_projects() -> Result<Vec<ProjectContract>> {
	let project_models = project_repository::get_all_projects().await?;

	let project_contracts: Vec<ProjectContract> = project_models
		.into_iter()
		.map(ProjectContract::from)
		.collect();

	Ok(project_contracts)
}

pub async fn get_project(id: &i32) -> Result<ProjectContract> {
	let project_model = project_repository::get_project(id).await?;

	let project_contract = ProjectContract::from(project_model);

	Ok(project_contract)
}
