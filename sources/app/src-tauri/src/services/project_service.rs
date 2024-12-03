use chrono::Utc;
use uuid::Uuid;

use crate::contracts::project_contracts::{CreateProjectContract, ProjectContract};
use crate::prelude::*;
use crate::repositories::project_repository;

pub async fn create_project(create_project_contract: &CreateProjectContract) -> Result<ProjectContract> {
	let date_created = Utc::now();
	let date_last_opened = Utc::now();

	let project_model = project_repository::create_project(&create_project_contract.name, &date_created, &date_last_opened).await?;

	let project_contract = ProjectContract::from(project_model);

	Ok(project_contract)
}

pub async fn get_all_projects() -> Result<Vec<ProjectContract>> {
	let project_models = project_repository::get_all_projects().await?;

	let project_contracts: Vec<ProjectContract> = project_models.into_iter().map(ProjectContract::from).collect();

	Ok(project_contracts)
}

pub async fn open_project(id: &Uuid) -> Result<ProjectContract> {
	let current_time = chrono::Utc::now();
	project_repository::update_project_last_opened(&id, &current_time).await?;

	let project_model = project_repository::get_project(id).await?;

	let project_contract = ProjectContract::from(project_model);

	Ok(project_contract)
}

pub async fn get_last_opened_project_id() -> Result<Option<Uuid>> {
	let last_opened_project_id = project_repository::get_last_opened_project_id().await?;

	Ok(last_opened_project_id)
}
