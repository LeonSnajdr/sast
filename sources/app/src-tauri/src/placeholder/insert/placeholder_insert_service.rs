use uuid::Uuid;

use crate::placeholder::insert::placeholder_insert_contracts::{PlaceholderInsertTileContract, PlaceholderInsertTileFilterContract};
use crate::placeholder::insert::placeholder_insert_models::{PlaceholderInsertTileFilterModel, PlaceholderInsertTileModel};
use crate::placeholder::insert::placeholder_insert_repository;
use crate::prelude::*;

pub async fn placeholder_insert_tile_create_or_replace(
	tile_contracts: Vec<PlaceholderInsertTileContract>,
	filter_contract: PlaceholderInsertTileFilterContract,
) -> Result<()> {
	let tile_models = tile_contracts
		.into_iter()
		.enumerate()
		.map(|(position, tile)| {
			let id = Uuid::new_v4();
			PlaceholderInsertTileModel::from(id, position as i64, filter_contract.task_command_id, filter_contract.task_working_dir_id, tile)
		})
		.collect();

	let filter_model = PlaceholderInsertTileFilterModel::from(filter_contract);

	placeholder_insert_repository::placeholder_insert_tile_create_or_replace(tile_models, filter_model).await?;

	Ok(())
}

pub async fn placeholder_insert_tile_get_many(filter_contract: PlaceholderInsertTileFilterContract) -> Result<Vec<PlaceholderInsertTileContract>> {
	let filter_model = PlaceholderInsertTileFilterModel::from(filter_contract);

	let tile_models = placeholder_insert_repository::placeholder_insert_tile_get_many(filter_model).await?;

	let tile_contracts = tile_models.into_iter().map(PlaceholderInsertTileContract::from).collect();

	Ok(tile_contracts)
}
