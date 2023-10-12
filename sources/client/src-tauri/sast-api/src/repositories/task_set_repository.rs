use prisma_client_rust::QueryError;

use crate::contracts::task_set_contracts::{
    full_task_set_contract, project_task_set_contract, CreateTaskSetContract,
};
use crate::prisma::{project, task_set};
use crate::utils::db_utils::DbState;

pub async fn create_task_set(
    db: DbState<'_>, create_contract: CreateTaskSetContract,
) -> Result<full_task_set_contract::Data, QueryError> {
    return db
        .task_set()
        .create(
            create_contract.name,
            project::id::equals(create_contract.project_id),
            vec![],
        )
        .include(full_task_set_contract::include())
        .exec()
        .await;
}

pub async fn get_project_task_set(
    db: DbState<'_>, task_set_id: String,
) -> Result<Option<project_task_set_contract::Data>, QueryError> {
    return db
        .task_set()
        .find_first(vec![task_set::id::equals(task_set_id)])
        .include(project_task_set_contract::include())
        .exec()
        .await;
}
