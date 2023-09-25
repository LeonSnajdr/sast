use prisma_client_rust::QueryError;

use crate::contracts::project_contracts::{
    full_project, CreateProjectContract, UpdateProjectContract,
};
use crate::prisma::project;
use crate::repositories::project_repository;
use crate::utils::db_utils::DbState;

pub async fn get_projects(db: DbState<'_>) -> Result<Vec<full_project::Data>, QueryError> {
    return project_repository::get_projects(db).await;
}

pub async fn create_project(
    db: DbState<'_>,
    create_contract: CreateProjectContract,
) -> Result<project::Data, QueryError> {
    return project_repository::create_project(db, create_contract).await;
}

pub async fn update_project(
    db: DbState<'_>,
    update_contract: UpdateProjectContract,
) -> Result<project::Data, QueryError> {
    return project_repository::update_project(db, update_contract).await;
}

pub async fn delete_project(
    db: DbState<'_>,
    project_id: String,
) -> Result<project::Data, QueryError> {
    return project_repository::delete_project(db, project_id).await;
}
