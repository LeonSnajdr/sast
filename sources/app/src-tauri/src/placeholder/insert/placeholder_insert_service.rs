use uuid::Uuid;

use crate::placeholder::insert::placeholder_insert_contracts::{PlaceholderInsertTileContract, PlaceholderInsertTileFilterContract};
use crate::placeholder::insert::placeholder_insert_enums::PlaceholderInsertTileKind;
use crate::placeholder::insert::placeholder_insert_models::{PlaceholderInsertTileFilterModel, PlaceholderInsertTileModel};
use crate::placeholder::insert::placeholder_insert_repository;
use crate::prelude::*;

pub async fn create_or_replace(tile_contracts: Vec<PlaceholderInsertTileContract>, filter_contract: PlaceholderInsertTileFilterContract) -> Result<()> {
	let tile_models = tile_contracts
		.into_iter()
		.enumerate()
		.map(|(position, tile)| {
			let id = Uuid::new_v4();
			PlaceholderInsertTileModel::from(id, position as i64, filter_contract.task_command_id, filter_contract.task_working_dir_id, tile)
		})
		.collect();

	let filter_model = PlaceholderInsertTileFilterModel::from(filter_contract);

	placeholder_insert_repository::create_or_replace(tile_models, filter_model).await?;

	Ok(())
}

pub async fn get_many(filter_contract: PlaceholderInsertTileFilterContract) -> Result<Vec<PlaceholderInsertTileContract>> {
	let filter_model = PlaceholderInsertTileFilterModel::from(filter_contract);

	let tile_models = placeholder_insert_repository::get_many(filter_model).await?;

	let tile_contracts = tile_models.into_iter().map(PlaceholderInsertTileContract::from).collect();

	Ok(tile_contracts)
}

pub async fn get_rendered_tiles(filter_contract: PlaceholderInsertTileFilterContract) -> Result<Option<String>> {
	let filter_model = PlaceholderInsertTileFilterModel::from(filter_contract);

	let tile_models = placeholder_insert_repository::get_many(filter_model).await?;

	if tile_models.len() == 0 {
		return Ok(None);
	}

	let mut result = String::new();

	for tile_model in tile_models {
		let value = match tile_model.kind {
			PlaceholderInsertTileKind::Text => tile_model.text_value,
			PlaceholderInsertTileKind::Placeholder => tile_model.placeholder_value,
		};

		if let Some(val) = value {
			result += &val;
		}
	}

	Ok(Some(result))
}
