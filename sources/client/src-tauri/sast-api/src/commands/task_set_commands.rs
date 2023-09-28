use prisma_client_rust::QueryError;
use specta;
use tauri;

use crate::contracts::task_set_contracts::CreateTaskSetContract;
use crate::prisma::task_set;
use crate::services::task_set_service;
use crate::utils::db_utils::DbState;

#[tauri::command]
#[specta::specta]
pub async fn create_task_set(
    db: DbState<'_>, create_contract: CreateTaskSetContract,
) -> Result<task_set::Data, QueryError> {
    return task_set_service::create_task_set(db, create_contract).await;
}

#[tauri::command]
#[specta::specta]
pub async fn start_task_set(db: DbState<'_>, task_set_id: String) -> Result<String, ()> {
    return task_set_service::start_task_set(db, task_set_id).await;
}
