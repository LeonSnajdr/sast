use prisma_client_rust::QueryError;

use crate::contracts::project_contracts::{
    full_project_contract, list_project_contract, CreateProjectContract, UpdateProjectContract,
};
use crate::prisma::project;
use crate::utils::db_utils::DbState;

pub async fn get_list_projects(
    db: DbState<'_>,
) -> Result<Vec<list_project_contract::Data>, QueryError> {
    let projects = db
        .project()
        .find_many(vec![])
        .select(list_project_contract::select())
        .exec()
        .await;

    return projects;
}

pub async fn get_full_project(
    db: DbState<'_>, project_id: String,
) -> Result<Option<full_project_contract::Data>, QueryError> {
    let project = db
        .project()
        .find_first(vec![project::id::equals(project_id)])
        .include(full_project_contract::include())
        .exec()
        .await;

    return project;
}

pub async fn create_project(
    db: DbState<'_>, create_contract: CreateProjectContract,
) -> Result<project::Data, QueryError> {
    return db
        .project()
        .create(create_contract.name, vec![])
        .exec()
        .await;
}

pub async fn update_project(
    db: DbState<'_>, update_contract: UpdateProjectContract,
) -> Result<project::Data, QueryError> {
    return db
        .project()
        .update(
            project::id::equals(update_contract.id),
            vec![project::name::set(update_contract.name)],
        )
        .exec()
        .await;
}

pub async fn delete_project(
    db: DbState<'_>, project_id: String,
) -> Result<project::Data, QueryError> {
    return db
        .project()
        .delete(project::id::equals(project_id))
        .exec()
        .await;
}
