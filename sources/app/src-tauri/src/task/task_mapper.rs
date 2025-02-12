use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::placeholder::insert::placeholder_insert_contracts::PlaceholderInsertTileContract;
use crate::placeholder::insert::placeholder_insert_models::PlaceholderInsertTileResultModel;
use crate::task::task_contracts::{TaskContract, TaskCreateContract, TaskInfoContract, TaskUpdateContract};
use crate::task::task_models::{TaskInfoModel, TaskModel, TaskUpdateModel};

impl TaskContract {
    pub fn from(command_tiles: Vec<PlaceholderInsertTileResultModel>, working_dir_tiles: Vec<PlaceholderInsertTileResultModel>, value: TaskModel) -> Self {
        Self {
            id: value.id,
            project_id: value.project_id,
            name: value.name,
            blocking: value.blocking,
            command_tiles: command_tiles.into_iter().map(PlaceholderInsertTileContract::from).collect(),
            working_dir_tiles: working_dir_tiles.into_iter().map(PlaceholderInsertTileContract::from).collect(),
            date_created: value.date_created,
            date_last_updated: value.date_last_updated
        }
    }
}

impl TaskInfoContract {
    pub fn from(value: TaskInfoModel) -> Self {
        Self {
            id: value.id,
            project_id: value.project_id,
            name: value.name,
            blocking: value.blocking,
            date_created: value.date_created,
            date_last_updated: value.date_last_updated
        }
    }
}

impl TaskModel {
    pub fn from_create_contract(id: Uuid, date_created: DateTime<Utc>, date_last_updated: DateTime<Utc>, value: TaskCreateContract) -> (Self, Vec<PlaceholderInsertTileContract>, Vec<PlaceholderInsertTileContract>) {
        let TaskCreateContract {
            project_id,
            name,
            blocking,
            command_tiles,
            working_dir_tiles,
        } = value;

        let task_model = Self {
            id,
            project_id,
            name,
            blocking,
            date_created,
            date_last_updated,
        };

       (task_model, command_tiles, working_dir_tiles)
    }
}

impl TaskUpdateModel {
    pub fn from(date_last_updated: DateTime<Utc>, value: TaskUpdateContract) -> Self {
        Self {
            id: value.id,
            name: value.name,
            blocking: value.blocking,
            date_last_updated
        }
    }
}