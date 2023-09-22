use prisma_client_rust::QueryError;

use crate::prisma::project;
use crate::utils::db_utils::DbState;
use crate::contracts::create_project_contract::CreateProjectContract;

pub async fn get_projects(db: DbState<'_>) -> Result<Vec<project::Data>, QueryError> {
    let projects = db.project().find_many(vec![]).exec().await;

    return projects;
}

pub async fn create_project(db: DbState<'_>, create_contract: CreateProjectContract) -> Result<project::Data, QueryError> {
    let data = db.project().create(create_contract.name,  vec![]).exec().await;

    return data;
}

pub async fn delete_project(db: DbState<'_>, project_id: String) {
    db.project().delete(project::id::equals(project_id)).exec().await.expect("Deleting project faild");
}