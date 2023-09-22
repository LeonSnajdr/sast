use tauri;
use specta;
use prisma_client_rust::QueryError;

use crate::prisma::project;
use crate::utils::db_utils::DbState;
use crate::contracts::create_project_contract::CreateProjectContract;
use crate::services::project_service;

#[tauri::command]
#[specta::specta]
pub async fn get_projects(db: DbState<'_>) -> Result<Vec<project::Data>, QueryError> {
    return project_service::get_projects(db).await;
}

#[tauri::command]
#[specta::specta]
pub async fn create_project(db: DbState<'_>, create_contract: CreateProjectContract) -> Result<project::Data, QueryError> {
    return project_service::create_project(db, create_contract).await;
}

#[tauri::command]
#[specta::specta]
pub async fn delete_project(db: DbState<'_>, project_id: String) -> Result<project::Data, QueryError>{
    return project_service::delete_project(db, project_id).await;
}