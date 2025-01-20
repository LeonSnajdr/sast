use chrono::Utc;
use uuid::Uuid;

use crate::prelude::*;
use crate::project::project_contracts::{ProjectCreateContract, ProjectContract};
use crate::project::project_repository;

pub async fn project_create(project_create_contract: &ProjectCreateContract) -> Result<ProjectContract> {
	let date_created = Utc::now();
	let date_last_opened = Utc::now();

	let project_model = project_repository::project_create(&project_create_contract.name, &date_created, &date_last_opened).await?;

	let project_contract = ProjectContract::from(project_model);

	Ok(project_contract)
}

pub async fn project_get_all() -> Result<Vec<ProjectContract>> {
	let project_models = project_repository::project_get_all().await?;

	let project_contracts: Vec<ProjectContract> = project_models.into_iter().map(ProjectContract::from).collect();

	Ok(project_contracts)
}

pub async fn open_project(id: &Uuid) -> Result<ProjectContract> {
	let current_time = chrono::Utc::now();
	project_repository::project_update_one(&id, &current_time).await?;

	let project_model = project_repository::project_get_one(id).await?;

	let project_contract = ProjectContract::from(project_model);

	Ok(project_contract)
}

pub async fn project_get_id_last_opened() -> Result<Option<Uuid>> {
	let last_opened_project_id = project_repository::project_get_id_last_opened().await?;

	Ok(last_opened_project_id)
}
