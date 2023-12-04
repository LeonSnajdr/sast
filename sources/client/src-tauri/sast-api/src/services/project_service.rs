use prisma_client_rust::QueryError;

use crate::contracts::project_contracts::{
    list_project_contract, CreateProjectContract, UpdateProjectContract,
};
use crate::prisma::project;
use crate::repositories::project_repository;
use crate::utils::db_utils::DbState;

#[tauri::command]
#[specta::specta]
pub async fn get_list_projects(
    db: DbState<'_>,
) -> Result<Vec<list_project_contract::Data>, QueryError> {
    return project_repository::get_list_projects(db).await;
}

#[tauri::command]
#[specta::specta]
pub async fn create_project(
    db: DbState<'_>, create_contract: CreateProjectContract,
) -> Result<list_project_contract::Data, QueryError> {
    return project_repository::create_project(db, create_contract).await;
}

#[tauri::command]
#[specta::specta]
pub async fn update_project(
    db: DbState<'_>, update_contract: UpdateProjectContract,
) -> Result<project::Data, QueryError> {
    return project_repository::update_project(db, update_contract).await;
}

#[tauri::command]
#[specta::specta]
pub async fn delete_project(
    db: DbState<'_>, project_id: String,
) -> Result<project::Data, QueryError> {
    return project_repository::delete_project(db, project_id).await;
}
