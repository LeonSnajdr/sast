use prisma_client_rust::QueryError;
use specta;
use tauri;

use crate::contracts::project_contracts::{
    full_project, CreateProjectContract, UpdateProjectContract,
};
use crate::prisma::project;
use crate::services::project_service;
use crate::utils::db_utils::DbState;

#[tauri::command]
#[specta::specta]
pub async fn get_projects(db: DbState<'_>) -> Result<Vec<full_project::Data>, QueryError> {
    return project_service::get_projects(db).await;
}

#[tauri::command]
#[specta::specta]
pub async fn create_project(
    db: DbState<'_>,
    create_contract: CreateProjectContract,
) -> Result<project::Data, QueryError> {
    return project_service::create_project(db, create_contract).await;
}

#[tauri::command]
#[specta::specta]
pub async fn update_project(
    db: DbState<'_>,
    update_contract: UpdateProjectContract,
) -> Result<project::Data, QueryError> {
    return project_service::update_project(db, update_contract).await;
}

#[tauri::command]
#[specta::specta]
pub async fn delete_project(
    db: DbState<'_>,
    project_id: String,
) -> Result<project::Data, QueryError> {
    return project_service::delete_project(db, project_id).await;
}
