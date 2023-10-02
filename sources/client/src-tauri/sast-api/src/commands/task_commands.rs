use prisma_client_rust::QueryError;
use specta;
use tauri;

use crate::contracts::task_contracts::{CreateTaskContract, UpdateTaskContract};
use crate::prisma::task;
use crate::services::task_service;
use crate::utils::db_utils::DbState;

#[tauri::command]
#[specta::specta]
pub async fn create_task(
    db: DbState<'_>, create_contract: CreateTaskContract,
) -> Result<task::Data, QueryError> {
    return task_service::create_task(db, create_contract).await;
}

#[tauri::command]
#[specta::specta]
pub async fn update_task(
    db: DbState<'_>, update_contract: UpdateTaskContract,
) -> Result<task::Data, QueryError> {
    return task_service::update_task(db, update_contract).await;
}

#[tauri::command]
#[specta::specta]
pub async fn delete_task(db: DbState<'_>, task_id: String) -> Result<task::Data, QueryError> {
    return task_service::delete_task(db, task_id).await;
}
