use uuid::Uuid;

use crate::prelude::*;
use crate::task::task_service;
use crate::task::task_contracts::{TaskCreateContract, TaskContract, TaskUpdateContract, TaskInfoContract};

#[tauri::command]
#[specta::specta]
pub async fn task_create(task_create_contract: TaskCreateContract) -> Result<Uuid> {
    let task_id = task_service::task_create(task_create_contract).await?;

    Ok(task_id)
}

#[tauri::command]
#[specta::specta]
pub async fn task_get_info_all_project(project_id: Uuid) -> Result<Vec<TaskInfoContract>> {
    let tasks = task_service::task_get_info_all_project(project_id).await?;

    Ok(tasks)
}

#[tauri::command]
#[specta::specta]
pub async fn task_get_one(id: Uuid) -> Result<TaskContract> {
    let task = task_service::task_get_one(id).await?;

    Ok(task)
}

#[tauri::command]
#[specta::specta]
pub async fn task_update_one(task_update_contract: TaskUpdateContract) -> Result<()> {
    task_service::task_update_one(task_update_contract).await?;

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn task_delete_one(id: Uuid) -> Result<()> {
    task_service::task_delete_one(id).await?;

    Ok(())
}
