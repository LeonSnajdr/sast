use tauri;
use specta;

use prisma_client_rust::QueryError;

use crate::prisma::project;
use crate::utils::db_utils::DbState;
use crate::contracts::create_project_contract::CreateProjectContract;

#[tauri::command]
#[specta::specta]
pub async fn get_projects(db: DbState<'_>) -> Result<Vec<project::Data>, QueryError> {
    db.project().find_many(vec![]).exec().await
}

#[tauri::command]
#[specta::specta]
pub async fn create_project(db: DbState<'_>, create_data: CreateProjectContract) -> Result<project::Data, ()> {
    let data: project::Data = db.project().create(create_data.name,  vec![]).exec().await.unwrap();

    return Ok(data);
}