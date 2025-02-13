use chrono::Utc;
use uuid::Uuid;

use crate::placeholder::insert::placeholder_insert_models::{PlaceholderInsertTileFilterModel, PlaceholderInsertTileModel};
use crate::prelude::*;
use crate::task::task_contracts::{TaskCreateContract, TaskContract, TaskUpdateContract, TaskInfoContract};
use crate::task::task_models::{TaskModel, TaskUpdateModel};
use crate::task::task_repository;
use crate::placeholder::insert::placeholder_insert_repository;

pub async fn task_create(task_create_contract: TaskCreateContract) -> Result<Uuid> {
    let id = Uuid::new_v4();
    let date_created = Utc::now();
    let date_last_updated = Utc::now();

    let (task_create_model, command_tiles, working_dir_tiles) = TaskModel::from_create_contract(id, date_created, date_last_updated, task_create_contract);

    let task_model = task_repository::task_create(task_create_model).await?;

    let delete_command_tiles_filter = PlaceholderInsertTileFilterModel {
        task_command_id: Some(task_model.id),
        ..PlaceholderInsertTileFilterModel::default()
    };

    let command_tile_models = command_tiles
        .into_iter()
        .enumerate()
        .map(|(position, command_tile)|
        {
            let id = Uuid::new_v4();
            PlaceholderInsertTileModel::from(id, position as i64, Some(task_model.id.clone()), None, command_tile)
        })
        .collect();

    placeholder_insert_repository::placeholder_insert_tile_update_many(command_tile_models, delete_command_tiles_filter).await?;

    let delete_working_dir_tiles_filter = PlaceholderInsertTileFilterModel {
        task_working_dir_id: Some(id),
        ..PlaceholderInsertTileFilterModel::default()
    };

    let working_dir_tiles_models = working_dir_tiles
        .into_iter()
        .enumerate()
        .map(|(position, working_dir_tile)|
            {
                let id = Uuid::new_v4();
                PlaceholderInsertTileModel::from(id, position as i64, None, Some(task_model.id), working_dir_tile)
            })
        .collect();

    placeholder_insert_repository::placeholder_insert_tile_update_many(working_dir_tiles_models, delete_working_dir_tiles_filter).await?;

    Ok(id)
}

pub async fn task_get_one(id: Uuid) -> Result<TaskContract> {
    let task_model = task_repository::task_get_one(id).await?;

    let command_tile_filter = PlaceholderInsertTileFilterModel {
        task_command_id: Some(id),
        ..PlaceholderInsertTileFilterModel::default()
    };

    let command_tiles = placeholder_insert_repository::placeholder_insert_tile_get_many(command_tile_filter).await?;

    let working_dir_tiles_filter = PlaceholderInsertTileFilterModel {
        task_working_dir_id: Some(id),
        ..PlaceholderInsertTileFilterModel::default()
    };

    let working_dir_tiles = placeholder_insert_repository::placeholder_insert_tile_get_many(working_dir_tiles_filter).await?;

    let task_contract = TaskContract::from(command_tiles, working_dir_tiles, task_model);

    Ok(task_contract)
}

pub async fn task_get_info_all_project(project_id: Uuid) -> Result<Vec<TaskInfoContract>> {
    let task_info_models = task_repository::task_get_info_all_project(project_id).await?;

    let task_info_contracts = task_info_models.into_iter().map(TaskInfoContract::from).collect();

    Ok(task_info_contracts)
}

pub async fn task_update_one(task_update_contract: TaskUpdateContract) -> Result<()> {
    let date_last_updated = Utc::now();

    let update_model = TaskUpdateModel::from(date_last_updated, task_update_contract);

    task_repository::task_update_one(update_model).await?;

    Ok(())
}

pub async fn task_delete_one(id: Uuid) -> Result<()> {
    task_repository::task_delete_one(id).await?;

    Ok(())
}
