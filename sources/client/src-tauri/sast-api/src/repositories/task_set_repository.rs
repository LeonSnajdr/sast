use prisma_client_rust::QueryError;

use crate::contracts::task_set_contracts::{start_task_set_contract, CreateTaskSetContract};
use crate::prisma::{project, task_set};
use crate::utils::db_utils::DbState;

pub async fn create_task_set(
    db: DbState<'_>, create_contract: CreateTaskSetContract,
) -> Result<task_set::Data, QueryError> {
    return db
        .task_set()
        .create(
            create_contract.name,
            project::id::equals(create_contract.project_id),
            vec![],
        )
        .exec()
        .await;
}

pub async fn get_task_set(
    db: DbState<'_>, task_set_id: String,
) -> Result<Option<start_task_set_contract::Data>, QueryError> {
    return db
        .task_set()
        .find_first(vec![task_set::id::equals(task_set_id)])
        .include(start_task_set_contract::include())
        .exec()
        .await;
}
