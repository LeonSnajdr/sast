use std::collections::HashMap;
use uuid::Uuid;

use crate::placeholder::insert::placeholder_insert_contracts::PlaceholderInsertTileContract;
use crate::placeholder::insert::placeholder_insert_enums::PlaceholderInsertTileKind;
use crate::placeholder::placeholder_contracts::PlaceholderContract;
use crate::placeholder::placeholder_service;
use crate::prelude::*;
use crate::task::task_contracts::{TaskCloneContract, TaskClonePlaceholderMappingContract, TaskContract, TaskCreateContract};
use crate::task::task_service;

pub async fn build_empty_task_clone(task_id: Uuid, project_id: Uuid) -> Result<TaskCloneContract> {
	let task_to_clone = task_service::get_one(task_id).await?;

	let task_placeholders = placeholder_service::get_many_used_in_task(task_id).await?;
	let project_placeholders = placeholder_service::get_many(project_id).await?;

	let placeholder_mappings = build_placeholder_mappings(&task_placeholders, &project_placeholders);

	let task_clone = TaskCloneContract {
		id: task_to_clone.id,
		new_name: task_to_clone.name + "_clone",
		new_project_id: project_id,
		task_placeholders,
		project_placeholders,
		placeholder_mappings,
	};

	Ok(task_clone)
}

pub fn build_placeholder_mappings(
	task_placeholders: &[PlaceholderContract],
	project_placeholders: &[PlaceholderContract],
) -> Vec<TaskClonePlaceholderMappingContract> {
	let project_map: HashMap<String, &PlaceholderContract> = project_placeholders.iter().map(|ph| (ph.name.to_lowercase(), ph)).collect();

	task_placeholders
		.iter()
		.map(|task_ph| {
			let to = project_map.get(&task_ph.name.to_lowercase()).map(|ph| (*ph).clone());
			TaskClonePlaceholderMappingContract { from: task_ph.clone(), to }
		})
		.collect()
}

pub async fn clone_one(clone_contract: TaskCloneContract) -> Result<TaskContract> {
	let task_to_clone = task_service::get_one(clone_contract.id).await?;

	let mut cloned_create_task = TaskCreateContract::from(clone_contract.new_project_id, clone_contract.new_name, task_to_clone);

	cloned_create_task.command_tiles = apply_placeholder_mappings(cloned_create_task.command_tiles, &clone_contract.placeholder_mappings);
	cloned_create_task.working_dir_tiles = apply_placeholder_mappings(cloned_create_task.working_dir_tiles, &clone_contract.placeholder_mappings);

	let cloned_task = task_service::create(cloned_create_task).await?;

	Ok(cloned_task)
}

fn apply_placeholder_mappings(
	tiles: Vec<PlaceholderInsertTileContract>,
	mappings: &[TaskClonePlaceholderMappingContract],
) -> Vec<PlaceholderInsertTileContract> {
	tiles
		.into_iter()
		.filter_map(|mut tile| {
			if tile.kind != PlaceholderInsertTileKind::Placeholder {
				return Some(tile);
			}

			let Some(placeholder_id) = tile.placeholder_id else {
				return None;
			};

			let Some(mapping) = mappings.iter().find(|m| m.from.id == placeholder_id) else {
				return None;
			};

			let Some(to) = mapping.to.as_ref() else {
				return None;
			};

			tile.placeholder_id = Some(to.id);

			Some(tile)
		})
		.collect()
}
