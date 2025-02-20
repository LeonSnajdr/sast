use chrono::Utc;
use uuid::Uuid;

use crate::placeholder::placeholder_contracts::{PlaceholderContract, PlaceholderCreateContract, PlaceholderUpdateContract};
use crate::placeholder::placeholder_models::{PlaceholderModel, PlaceholderUpdateModel};
use crate::placeholder::placeholder_repository;
use crate::prelude::*;

pub async fn create(placeholder_create_contract: PlaceholderCreateContract) -> Result<PlaceholderContract> {
	let id = Uuid::new_v4();
	let date_created = Utc::now();
	let date_last_updated = Utc::now();

	let create_model = PlaceholderModel::from_create_contract(id, date_created, date_last_updated, placeholder_create_contract);

	let placeholder_model = placeholder_repository::create(create_model).await?;

	let placeholder_contract = PlaceholderContract::from(placeholder_model);

	Ok(placeholder_contract)
}

pub async fn get_many(project_id: Uuid) -> Result<Vec<PlaceholderContract>> {
	let placeholder_models = placeholder_repository::get_many(project_id).await?;

	let placeholder_contracts = placeholder_models.into_iter().map(PlaceholderContract::from).collect();

	Ok(placeholder_contracts)
}
pub async fn get_one(id: Uuid) -> Result<PlaceholderContract> {
	let placeholder_model = placeholder_repository::get_one(id).await?;

	let placeholder_contract = PlaceholderContract::from(placeholder_model);

	Ok(placeholder_contract)
}

pub async fn update_one(placeholder_update_contract: PlaceholderUpdateContract) -> Result<()> {
	let date_last_updated = Utc::now();

	let update_container = PlaceholderUpdateModel::from(date_last_updated, placeholder_update_contract);

	placeholder_repository::update_one(update_container).await?;

	Ok(())
}

pub async fn delete_one(id: Uuid) -> Result<()> {
	placeholder_repository::delete_one(id).await?;

	Ok(())
}
