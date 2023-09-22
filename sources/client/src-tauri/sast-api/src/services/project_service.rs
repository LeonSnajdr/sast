use prisma_client_rust::QueryError;

use crate::prisma::project;
use crate::utils::db_utils::DbState;
use crate::contracts::project_contracts::{CreateProjectContract, UpdateProjectContract};

pub async fn get_projects(db: DbState<'_>) -> Result<Vec<project::Data>, QueryError> {
    let projects = db
        .project()
        .find_many(vec![])
        .select(project::select!({ placeholders }))
        .exec()
        .await;

    return projects;
}

pub async fn create_project(db: DbState<'_>, create_contract: CreateProjectContract) -> Result<project::Data, QueryError> {
    return db
        .project()
        .create(
            create_contract.name,
            vec![]
        )
        .exec()
        .await;
}

pub async fn update_project(db: DbState<'_>, update_contract: UpdateProjectContract) -> Result<project::Data, QueryError> {
    return db
        .project()
        .update(
        project::id::equals(update_contract.id),
        vec![project::name::set(update_contract.name)]
        )
        .exec()
        .await
}

pub async fn delete_project(db: DbState<'_>, project_id: String) -> Result<project::Data, QueryError> {
    return db
        .project()
        .delete(project::id::equals(project_id))
        .exec()
        .await;
}