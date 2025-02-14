use chrono::Utc;
use uuid::Uuid;
use crate::placeholder::insert::placeholder_insert_contracts::PlaceholderInsertTileFilterContract;
use crate::prelude::*;
use crate::placeholder::insert::placeholder_insert_service;
use crate::task::task_contracts::{TaskCreateContract, TaskContract, TaskUpdateContract, TaskInfoContract};
use crate::task::task_models::{TaskModel, TaskUpdateModel};
use crate::task::task_repository;

pub async fn task_create(task_create_contract: TaskCreateContract) -> Result<Uuid> {
    let id = Uuid::new_v4();
    let date_created = Utc::now();
    let date_last_updated = Utc::now();

    let (task_create_model, command_tiles, working_dir_tiles) = TaskModel::from_create_contract(id, date_created, date_last_updated, task_create_contract);

    task_repository::task_create(task_create_model).await?;

    let delete_command_tiles_filter = PlaceholderInsertTileFilterContract{
        task_command_id: Some(id),
        ..PlaceholderInsertTileFilterContract::default()
    };

    placeholder_insert_service::placeholder_insert_tile_create_or_replace(command_tiles, delete_command_tiles_filter).await?;

    let delete_working_dir_tiles_filter = PlaceholderInsertTileFilterContract {
        task_working_dir_id: Some(Uuid::new_v4()),
        ..PlaceholderInsertTileFilterContract::default()
    };

    placeholder_insert_service::placeholder_insert_tile_create_or_replace(working_dir_tiles, delete_working_dir_tiles_filter).await?;

    Ok(id)
}

pub async fn task_get_one(id: Uuid) -> Result<TaskContract> {
    let task_model = task_repository::task_get_one(id).await?;

    let command_tile_filter = PlaceholderInsertTileFilterContract {
        task_command_id: Some(id),
        ..PlaceholderInsertTileFilterContract::default()
    };

    let command_tile_contracts = placeholder_insert_service::placeholder_insert_tile_get_many(command_tile_filter).await?;

    let working_dir_tile_filter = PlaceholderInsertTileFilterContract {
        task_working_dir_id: Some(id),
        ..PlaceholderInsertTileFilterContract::default()
    };

    let working_dir_tile_contracts = placeholder_insert_service::placeholder_insert_tile_get_many(working_dir_tile_filter).await?;

    let task_contract = TaskContract::from(command_tile_contracts, working_dir_tile_contracts, task_model);

    Ok(task_contract)
}

pub async fn task_get_info_all_project(project_id: Uuid) -> Result<Vec<TaskInfoContract>> {
    let task_info_models = task_repository::task_get_info_all_project(project_id).await?;

    let task_info_contracts = task_info_models.into_iter().map(TaskInfoContract::from).collect();

    Ok(task_info_contracts)
}

pub async fn task_update_one(task_update_contract: TaskUpdateContract) -> Result<()> {
    let date_last_updated = Utc::now();

    let (task_update_model, command_tiles, working_dir_tiles) = TaskUpdateModel::from(date_last_updated, task_update_contract);

    let delete_command_tiles_filter = PlaceholderInsertTileFilterContract{
        task_command_id: Some(task_update_model.id),
        ..PlaceholderInsertTileFilterContract::default()
    };

    placeholder_insert_service::placeholder_insert_tile_create_or_replace(command_tiles, delete_command_tiles_filter).await?;

    let delete_working_dir_tiles_filter = PlaceholderInsertTileFilterContract {
        task_working_dir_id: Some(task_update_model.id),
        ..PlaceholderInsertTileFilterContract::default()
    };

    placeholder_insert_service::placeholder_insert_tile_create_or_replace(working_dir_tiles, delete_working_dir_tiles_filter).await?;

    task_repository::task_update_one(task_update_model).await?;

    Ok(())
}

pub async fn task_delete_one(id: Uuid) -> Result<()> {
    task_repository::task_delete_one(id).await?;

    Ok(())
}
